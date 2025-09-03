# WASM Plugin System Architecture

## Overview

IdeaNet uses WebAssembly (WASM) for runtime plugin loading to achieve:
- **Security**: Sandboxed execution environment
- **Cross-platform**: Single binary works on all platforms
- **Performance**: Near-native execution speed
- **Simplicity**: No dynamic linking complexity

## Plugin Architecture

### WASM Plugin Interface

```rust
// Host-side plugin management
pub struct WasmPluginRunner {
    engine: wasmtime::Engine,
    plugins: HashMap<String, PluginInstance>,
}

pub struct PluginInstance {
    store: wasmtime::Store<PluginContext>,
    instance: wasmtime::Instance,
    exports: PluginExports,
}

// Plugin exports (functions the host can call)
pub struct PluginExports {
    pub initialize: wasmtime::TypedFunc<(u32, u32), u32>,
    pub handle_command: wasmtime::TypedFunc<(u32, u32), u32>,
    pub shutdown: wasmtime::TypedFunc<(), u32>,
}
```

### Plugin Development Kit (PDK)

```rust
// Plugin-side development kit
use ideanet_plugin_sdk::*;

#[plugin_main]
pub struct MyPlugin {
    host_api: HostApi,
}

impl Plugin for MyPlugin {
    fn initialize(&mut self, host_api: HostApi) -> Result<(), PluginError> {
        self.host_api = host_api;
        
        // Register UI components
        self.host_api.ui_registry.register_block_type(BlockTypeConfig {
            id: "my-custom-block",
            name: "My Custom Block",
            component: "MyCustomBlockComponent",
        })?;
        
        // Register event handlers
        self.host_api.event_bus.subscribe("content.created", |event| {
            // Handle content creation
        })?;
        
        Ok(())
    }
    
    fn handle_command(&mut self, command: Command) -> Result<CommandResponse, PluginError> {
        match command.action.as_str() {
            "render_block" => self.render_custom_block(command.payload),
            "process_data" => self.process_data(command.payload),
            _ => Err(PluginError::UnknownCommand(command.action)),
        }
    }
}
```

## Frontend Integration

### React Plugin Components

```typescript
// Frontend plugin registry
export class PluginRegistry {
  private components = new Map<string, React.ComponentType<any>>();
  private plugins = new Map<string, PluginManifest>();
  
  async loadPlugin(manifest: PluginManifest): Promise<void> {
    // Load WASM module
    const wasmModule = await WebAssembly.instantiateStreaming(
      fetch(manifest.wasm_url)
    );
    
    // Load frontend assets if they exist
    if (manifest.frontend_assets) {
      const assets = await import(manifest.frontend_assets);
      
      // Register React components
      for (const [name, component] of Object.entries(assets.components)) {
        this.components.set(`${manifest.id}.${name}`, component as React.ComponentType);
      }
    }
    
    // Initialize backend plugin
    await invoke('plugin_load', { 
      pluginId: manifest.id, 
      wasmBytes: await wasmModule.arrayBuffer() 
    });
  }
}

// Plugin component wrapper
export function PluginComponent({ pluginId, componentName, ...props }: PluginComponentProps) {
  const Component = usePlugin(pluginId, componentName);
  
  if (!Component) {
    return <div>Plugin component not found: {pluginId}.{componentName}</div>;
  }
  
  return <Component {...props} />;
}
```

## Plugin Marketplace

### Discovery and Installation

```rust
// Plugin store service
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginStoreService {
    registry_url: String,
    cache: PluginCache,
    security: SecurityManager,
}

impl PluginStoreService {
    pub async fn discover_plugins(&self, query: Option<&str>) -> Result<Vec<PluginManifest>, StoreError> {
        let url = format!("{}/plugins/search", self.registry_url);
        let response: PluginSearchResponse = reqwest::get(&url).await?.json().await?;
        
        // Filter by query if provided
        let plugins = if let Some(q) = query {
            response.plugins.into_iter()
                .filter(|p| p.name.contains(q) || p.description.contains(q))
                .collect()
        } else {
            response.plugins
        };
        
        Ok(plugins)
    }
    
    pub async fn install_plugin(&mut self, plugin_id: &str) -> Result<(), StoreError> {
        // 1. Download plugin manifest
        let manifest = self.get_plugin_manifest(plugin_id).await?;
        
        // 2. Verify signature for security
        self.security.verify_plugin_signature(&manifest)?;
        
        // 3. Download WASM binary
        let wasm_bytes = self.download_plugin_wasm(&manifest).await?;
        
        // 4. Load and initialize plugin
        let plugin_runner = WasmPluginRunner::new();
        plugin_runner.load_plugin(plugin_id, &wasm_bytes).await?;
        
        // 5. Update installed plugins registry
        self.cache.mark_installed(plugin_id, &manifest).await?;
        
        Ok(())
    }
}
```

## Security Model

### Sandboxing and Permissions

```rust
pub struct PluginSecurity {
    capabilities: HashSet<PluginCapability>,
    resource_limits: ResourceLimits,
}

pub struct ResourceLimits {
    max_memory: usize,
    max_cpu_time: Duration,
    allowed_network_domains: Vec<String>,
    file_access_scope: FileAccessScope,
}

pub enum FileAccessScope {
    None,
    ReadOnly(PathBuf),
    ReadWrite(PathBuf),
}

// Plugin capability system
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum PluginCapability {
    // Block system
    RegisterBlockType,
    RenderBlocks,
    
    // Event system
    PublishEvents(Vec<String>),
    SubscribeEvents(Vec<String>),
    
    // Storage
    ReadUserData,
    WriteUserData,
    
    // Network
    HttpRequest(Vec<String>), // allowed domains
    
    // System (require explicit user permission)
    RecordAudio,
    AccessClipboard,
    RunTerminalCommands,
}
```

## Development Workflow

### Plugin Development

1. **Setup**: Use `ideanet-plugin-template` crate
2. **Development**: Write plugin using PDK
3. **Build**: Compile to WASM target
4. **Test**: Local plugin testing tools
5. **Publish**: Submit to plugin registry

```bash
# Plugin development workflow
cargo new my-ideanet-plugin --lib
cd my-ideanet-plugin

# Add to Cargo.toml
[lib]
crate-type = ["cdylib"]

[dependencies]
ideanet-plugin-sdk = "0.1.0"

# Build for WASM
cargo build --target wasm32-wasi --release

# Test locally
ideanet-dev test-plugin target/wasm32-wasi/release/my_ideanet_plugin.wasm

# Publish
ideanet-dev publish --plugin-manifest plugin.toml
```

## Benefits of This Approach

### For Users
- **Easy Installation**: One-click plugin installation from marketplace
- **Security**: Sandboxed plugins can't harm system
- **Performance**: Near-native performance
- **Reliability**: Plugin crashes don't crash the app

### For Plugin Developers
- **Simple SDK**: Clear APIs and documentation
- **Cross-platform**: Write once, run everywhere
- **Hot Reload**: Fast development iteration
- **Distribution**: Easy publishing to marketplace

### For IdeaNet
- **Security**: Controlled plugin execution environment
- **Maintainability**: Clear plugin boundaries
- **Extensibility**: Rich plugin ecosystem potential
- **Monetization**: Potential plugin marketplace revenue
