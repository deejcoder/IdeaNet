# IdeaNet - Frontend Application Specification v2

## 1. Overview

IdeaNet is a graph-based knowledge management desktop application where ideas are represented as interconnected nodes with expandable functionality. Built with Tauri, React, and TypeScript, it provides intelligent project management with embedded resources, AI-guided suggestions, and rich multimedia capture including audio recording and transcription.

### Frontend Architecture
The IdeaNet frontend is built as a modern React application with the following key technologies:
- **React 18** with TypeScript for type-safe component development
- **Chakra UI** for consistent, accessible design system
- **Tauri** for desktop application capabilities and native integrations
- **React Flow** for graph visualization and node interaction
- **React Query** for server state management and API synchronization
- **Zustand** for client-side state management
- **WASM Runtime** for plugin system integration

### Frontend Modules:
1. **`ideanet-ui`** - React frontend with graph canvas and node management
2. **`ideanet-nodes`** - Node focus, expansion, and interaction system
3. **`ideanet-audio-ui`** - Audio recording, playback, and transcription interfaces
4. **`ideanet-ai-ui`** - AI integration UI for suggestions and content generation
5. **`ideanet-plugins-ui`** - Plugin management and execution interfaces
6. **`ideanet-graph-ui`** - Graph visualization, navigation, and layout management

*Note: Backend data management and APIs are detailed in [IdeaNet Backend API Specification v2](./ideanet-api-v2.md).*

## 2. Executive Summary

IdeaNet transforms knowledge management into a visual, interconnected experience where users create and explore graphs of connected ideas. The frontend handles all user interaction, node focus management, content expansion, and plugin execution while maintaining clean separation from backend data concerns.

### Key Frontend Capabilities
- **Graph Canvas Interface**: Interactive node-based knowledge visualization
- **Node Focus System**: Contextual expansion and content interaction
- **Plugin Ecosystem**: WASM-based extensible node types and functionality
- **Audio-First Capture**: Seamless voice recording and transcription
- **AI Integration**: Smart suggestions, content analysis, and connection discovery
- **Offline-First Design**: Local state with background synchronization

## 3. Technology Stack

### Core Technologies
- **React 18** - Component framework with concurrent features
- **TypeScript 5** - Type safety and developer experience
- **Tauri 1.5** - Desktop app framework with Rust backend
- **Chakra UI 3** - Design system and component library
- **React Flow** - Interactive graph visualization library
- **React Query** - Server state management and caching
- **Zustand** - Client-side state management
- **wasmtime** - WASM plugin execution environment

### UI Architecture
```
┌─────────────────────────────────────────┐
│         Desktop Application            │
│  ┌─────────────────────────────────────┐│
│  │      React Frontend                 ││
│  │   - Graph Canvas (React Flow)       ││
│  │   - Node Components (Chakra UI)     ││
│  │   - Plugin Runtime (WASM)           ││
│  │   - AI Integration (API calls)      ││
│  └─────────────────────────────────────┘│
│  ┌─────────────────────────────────────┐│
│  │      Tauri Runtime                  ││
│  │   - Native OS integration           ││
│  │   - File system access              ││
│  │   - Audio capture/playback          ││
│  │   - Window management               ││
│  └─────────────────────────────────────┘│
│  ┌─────────────────────────────────────┐│
│  │      Local Storage                  ││
│  │   - IndexedDB for offline data      ││
│  │   - File cache for media            ││
│  │   - Plugin binaries                 ││
│  └─────────────────────────────────────┘│
└─────────────────────────────────────────┘
```

---

## 4. Core User Interface Components

### 4.1 Graph Canvas

The main interface is a zoomable, pannable canvas where nodes represent ideas and edges represent connections.

#### Canvas Features
- **Infinite Canvas**: Smooth zooming and panning
- **Node Positioning**: Drag-and-drop node positioning
- **Connection Drawing**: Visual connection creation between nodes
- **Selection System**: Single and multi-node selection
- **Layout Algorithms**: Force-directed, hierarchical, and custom layouts

#### Canvas State Management
```typescript
interface CanvasState {
  viewport: {
    x: number;
    y: number;
    zoom: number;
  };
  selectedNodes: string[];
  focusedNode: string | null;
  connectionMode: boolean;
  layoutType: 'force-directed' | 'hierarchical' | 'manual';
}
```

### 4.2 Node System

Nodes are the core interactive elements representing different types of content.

#### Node States
- **Minimized**: Compact view showing only title/icon
- **Focused**: Selected node with expanded controls
- **Expanded**: Full content view with plugin-specific interface

#### Node Component Structure
```typescript
interface NodeProps {
  node: Node;
  isFocused: boolean;
  isSelected: boolean;
  expansionMode: 'minimized' | 'focused' | 'expanded';
  onFocusChange: (nodeId: string, focused: boolean) => void;
  onExpansionChange: (nodeId: string, mode: string) => void;
  onDataChange: (nodeId: string, data: any) => void;
}

interface Node {
  id: string;
  nodeType: string;
  position: { x: number; y: number };
  size: { width: number; height: number };
  data: Record<string, any>;
  pluginId: string;
  expansionCapabilities: string[];
}
```

#### Core Node Types

**Text Node**
- **Minimized**: Title preview with word count
- **Focused**: Inline editing with formatting toolbar
- **Expanded**: Full rich text editor with collaboration cursor

**Audio Node** 
- **Minimized**: Audio icon with duration
- **Focused**: Play/pause controls with waveform preview
- **Expanded**: Full waveform, transcript, and editing tools

**File/Resource Node**
- **Minimized**: File icon with name and type
- **Focused**: Preview thumbnail with metadata
- **Expanded**: Full file viewer or external app launcher

### 4.3 Node Focus Management

The focus system determines which node receives user attention and how content is displayed.

#### Focus Rules
1. **Single Focus**: Only one node can be focused at a time
2. **Context Awareness**: Focused node affects surrounding node visibility
3. **Expansion Triggers**: Click, keyboard navigation, or programmatic focus
4. **Auto-collapse**: Previously focused nodes return to minimized state

#### Focus State Management
```typescript
interface FocusManager {
  focusedNodeId: string | null;
  expandedNodes: Map<string, string>; // nodeId -> expansionMode
  contextNodes: string[]; // nodes visible due to focus context
  
  focusNode(nodeId: string): void;
  expandNode(nodeId: string, mode: string): void;
  collapseNode(nodeId: string): void;
  setContext(nodeIds: string[]): void;
}
```

#### Expansion Modes by Node Type
```typescript
const NodeExpansionModes = {
  text: ['preview', 'edit', 'fullscreen'],
  audio: ['play', 'transcript', 'waveform', 'edit'],
  file: ['preview', 'metadata', 'external'],
  code: ['preview', 'execute', 'debug'],
  image: ['thumbnail', 'fullview', 'annotate']
};
```

### 4.4 Plugin System UI

Plugins extend functionality by providing custom node types and expansion modes.

#### Plugin Integration
```typescript
interface PluginUI {
  pluginId: string;
  name: string;
  nodeTypes: string[];
  
  renderNode(props: NodeProps): ReactElement;
  renderExpansion(node: Node, mode: string): ReactElement;
  getExpansionModes(node: Node): string[];
  handleNodeAction(action: string, node: Node): void;
}
```

#### Plugin Manager Interface
- **Plugin Library**: Browse and install community plugins
- **Plugin Settings**: Configure plugin preferences and permissions
- **Plugin Development**: Tools for creating custom plugins
- **Security Indicators**: Show plugin capabilities and permissions

### 4.5 Audio Integration

Audio recording and transcription are first-class features integrated throughout the interface.

#### Audio Recording Interface
- **Global Record Button**: Quick capture from anywhere in the app
- **Node-Specific Recording**: Record directly into audio nodes
- **Background Recording**: Continue recording while navigating
- **Gesture Controls**: Push-to-talk and voice activation

#### Audio Node Features
```typescript
interface AudioNodeData {
  fileId: string;
  transcript?: string;
  duration: number;
  waveformPeaks: number[];
  chapters?: Array<{
    title: string;
    startTime: number;
    endTime: number;
  }>;
  analysis?: {
    sentiment: number;
    topics: string[];
    keyPhrases: string[];
  };
}
```

### 4.6 AI Integration UI

AI features provide intelligent suggestions and content analysis throughout the interface.

#### AI Suggestion Panel
- **Connection Suggestions**: Proposed relationships between nodes
- **Content Suggestions**: AI-generated content based on context
- **Research Assistance**: Fact-checking and related information
- **Summarization**: Automatic summaries of complex content

#### AI Controls
```typescript
interface AIControls {
  enableSuggestions: boolean;
  suggestionTypes: ('connections' | 'content' | 'research')[];
  confidenceThreshold: number;
  autoApply: boolean;
}
```

---

## 5. User Experience Flows

### 5.1 Creating and Connecting Ideas

#### Flow: Voice Idea Capture
1. **User presses global record hotkey** (e.g., Cmd+R)
2. **Recording indicator appears** in top-right corner
3. **User speaks their idea** while continuing other work
4. **User presses hotkey again** to stop recording
5. **Audio node appears** at cursor location or canvas center
6. **Transcription begins** automatically in background
7. **AI suggests connections** to existing nodes

#### Flow: Text Idea Development
1. **User creates text node** via canvas right-click or hotkey
2. **Node appears in minimized state** with focus
3. **User types title** in inline edit mode
4. **User presses Enter** to expand to full editor
5. **Rich text editing** with markdown shortcuts
6. **AI suggests related content** as user types
7. **User creates connections** by dragging to other nodes

### 5.2 Exploring Knowledge Graphs

#### Flow: Node Exploration
1. **User clicks on a node** to focus it
2. **Node expands** to show relevant expansion mode
3. **Connected nodes become highlighted** with connection strength
4. **Context panel shows** related suggestions and metadata
5. **User can traverse** to connected nodes via keyboard or click
6. **Previous node collapses** automatically unless pinned

#### Flow: Semantic Discovery
1. **User searches** for content using natural language
2. **AI returns semantically similar** nodes and files
3. **Search results show** as temporary nodes on canvas
4. **User can connect** search results to existing graph
5. **AI suggests** additional related content from knowledge base

### 5.3 Plugin-Enhanced Workflows

#### Flow: Installing a Plugin
1. **User opens plugin marketplace** from settings menu
2. **Browse or search** available plugins
3. **Plugin details show** capabilities and permissions
4. **User clicks install** and confirms permissions
5. **WASM binary downloads** and validates
6. **Plugin appears** in node creation menu
7. **User creates node** with new plugin type

#### Flow: Custom Node Interaction
1. **User creates plugin node** (e.g., mindmap, code, chart)
2. **Plugin renders** custom interface in expanded mode
3. **User interacts** with plugin-specific controls
4. **Plugin processes** user input in sandboxed environment
5. **Results update** node data and trigger re-render
6. **Changes sync** to backend with plugin data

---

## 6. State Management Architecture

### 6.1 Global State (Zustand)

```typescript
interface AppState {
  // Canvas state
  canvas: CanvasState;
  setCanvasViewport: (viewport: Viewport) => void;
  setSelectedNodes: (nodeIds: string[]) => void;
  
  // Focus management
  focusedNodeId: string | null;
  expandedNodes: Map<string, string>;
  setFocusedNode: (nodeId: string | null) => void;
  setNodeExpansion: (nodeId: string, mode: string) => void;
  
  // UI state
  sidebarOpen: boolean;
  searchOpen: boolean;
  aiPanelOpen: boolean;
  recordingActive: boolean;
  
  // Plugin state
  installedPlugins: Plugin[];
  activePlugins: string[];
  
  // Settings
  preferences: UserPreferences;
}
```

### 6.2 Server State (React Query)

```typescript
// Project data
const useProject = (projectId: string) => 
  useQuery(['project', projectId], () => api.getProject(projectId));

const useProjectGraph = (projectId: string) => 
  useQuery(['graph', projectId], () => api.getProjectGraph(projectId));

// Node operations
const useCreateNode = () => 
  useMutation(api.createNode, {
    onSuccess: (node) => {
      queryClient.invalidateQueries(['graph', node.projectId]);
    }
  });

const useUpdateNode = () => 
  useMutation(api.updateNode, {
    onSuccess: (node) => {
      queryClient.setQueryData(['node', node.id], node);
    }
  });

// Search and AI
const useSemanticSearch = (query: string) => 
  useQuery(['search', query], () => api.semanticSearch(query), {
    enabled: query.length > 3,
    staleTime: 30000
  });
```

### 6.3 Offline-First Architecture

```typescript
interface OfflineManager {
  isOnline: boolean;
  pendingChanges: ChangeQueue;
  lastSyncTime: Date | null;
  
  queueChange(change: Change): void;
  syncPendingChanges(): Promise<void>;
  handleConflict(conflict: Conflict): Promise<void>;
}

interface Change {
  id: string;
  type: 'create' | 'update' | 'delete';
  entityType: 'node' | 'connection' | 'project';
  entityId: string;
  data: any;
  timestamp: Date;
}
```

---

## 7. Component Architecture

### 7.1 Core Components

#### GraphCanvas
```typescript
interface GraphCanvasProps {
  projectId: string;
  nodes: Node[];
  connections: Connection[];
  onNodeCreate: (type: string, position: Position) => void;
  onNodeUpdate: (nodeId: string, updates: Partial<Node>) => void;
  onConnectionCreate: (from: string, to: string) => void;
}
```

#### NodeRenderer
```typescript
interface NodeRendererProps {
  node: Node;
  isSelected: boolean;
  isFocused: boolean;
  expansionMode: string;
  plugin: Plugin;
  onFocusChange: (focused: boolean) => void;
  onDataChange: (data: any) => void;
}
```

#### FocusManager
```typescript
interface FocusManagerProps {
  children: ReactNode;
  onFocusChange: (nodeId: string | null) => void;
  onExpansionChange: (nodeId: string, mode: string) => void;
}
```

### 7.2 Plugin Components

#### PluginRenderer
```typescript
interface PluginRendererProps {
  plugin: Plugin;
  node: Node;
  mode: 'minimized' | 'focused' | 'expanded';
  onDataChange: (data: any) => void;
}
```

#### PluginSandbox
```typescript
interface PluginSandboxProps {
  pluginId: string;
  wasmBinary: Uint8Array;
  capabilities: string[];
  onMessage: (message: any) => void;
}
```

### 7.3 AI Components

#### AIPanel
```typescript
interface AIPanelProps {
  isOpen: boolean;
  focusedNode: Node | null;
  suggestions: AISuggestion[];
  onSuggestionApply: (suggestion: AISuggestion) => void;
  onClose: () => void;
}
```

#### SmartSearch
```typescript
interface SmartSearchProps {
  onResultSelect: (result: SearchResult) => void;
  onCreateConnection: (from: string, to: string) => void;
}
```

---

## 8. User Stories & Implementation Plan

### Phase 1: Graph Foundation (Weeks 1-4)
- **UI-001**: Basic graph canvas with React Flow integration
- **UI-002**: Node creation, selection, and basic interaction
- **UI-003**: Connection creation and visualization
- **UI-004**: Simple text and file node types
- **Deliverable**: Working graph interface with basic node types

### Phase 2: Node Focus System (Weeks 5-8)
- **UI-005**: Node focus management and state transitions
- **UI-006**: Expansion modes and contextual interfaces
- **UI-007**: Keyboard navigation and shortcuts
- **UI-008**: Context-aware node visibility
- **Deliverable**: Rich node interaction and focus system

### Phase 3: Audio Integration (Weeks 9-12)
- **UI-009**: Audio recording interface and controls
- **UI-010**: Audio node with waveform visualization
- **UI-011**: Transcription display and editing
- **UI-012**: Audio chapter and annotation system
- **Deliverable**: Comprehensive audio capture and playback

### Phase 4: AI Integration (Weeks 13-16)
- **UI-013**: AI suggestion panel and controls
- **UI-014**: Semantic search interface
- **UI-015**: Smart connection recommendations
- **UI-016**: Content analysis and summarization
- **Deliverable**: AI-powered knowledge discovery

### Phase 5: Plugin System (Weeks 17-20)
- **UI-017**: Plugin marketplace and installation
- **UI-018**: WASM plugin runtime integration
- **UI-019**: Plugin security and permission system
- **UI-020**: Plugin development tools and debugging
- **Deliverable**: Extensible plugin ecosystem

**Future Enhancements:**
- 🔮 Real-time collaboration cursors and presence
- 🔮 Advanced graph analytics and visualization
- 🔮 Multi-modal AI integration (image, video analysis)
- 🔮 Export formats and presentation modes

---

## 9. Technical Specifications

### 9.1 Performance Requirements
- **Canvas Rendering**: 60fps with 1000+ nodes
- **Audio Latency**: <100ms for recording start/stop
- **Search Response**: <500ms for semantic search
- **Plugin Execution**: <1s for most operations
- **Offline Sync**: Background sync without UI blocking

### 9.2 Accessibility
- **Keyboard Navigation**: Full app navigable via keyboard
- **Screen Reader Support**: Semantic HTML and ARIA labels
- **High Contrast**: Support for system dark/light themes
- **Voice Control**: Audio recording via voice commands
- **Customizable UI**: Adjustable font sizes and spacing

### 9.3 Security Model
- **Plugin Sandboxing**: WASM plugins cannot access OS directly
- **Capability System**: Plugins request specific permissions
- **Data Isolation**: Plugin data cannot access other plugin data
- **File Access**: Controlled file system access via Tauri APIs
- **Network Requests**: Plugin network access is capability-gated

This frontend specification focuses on user experience, interaction patterns, and visual interfaces while maintaining clean separation from backend data concerns. The graph-first approach with React Flow provides an intuitive foundation for knowledge management while the plugin system enables unlimited extensibility.
