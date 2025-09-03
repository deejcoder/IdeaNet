# Development Guidelines for IdeaNet

## Terminal Commands Best Practices

### PowerShell Directory Navigation Rule

**CRITICAL: Always use full absolute paths with semicolon command chaining**

❌ **WRONG - This doesn't work because each run_in_terminal starts a new session:**
```powershell
cd directory    # First terminal session
command         # NEW terminal session - loses directory!
```

✅ **CORRECT - Use full path with semicolon in single command:**
```powershell
cd C:\Users\TonksD\git\github.com\deejcoder\IdeaNet\ideanet; yarn tauri dev
```

### Key Points:
- PowerShell uses `;` (semicolon) not `&&` for command chaining
- `run_in_terminal` tool creates fresh terminal sessions
- Directory changes don't persist between terminal calls
- Always use absolute paths to avoid confusion

### Project Paths:
- **Main Project Root:** `C:\Users\TonksD\git\github.com\deejcoder\IdeaNet`
- **Tauri Project Root:** `C:\Users\TonksD\git\github.com\deejcoder\IdeaNet\ideanet`
- **Rust Source:** `C:\Users\TonksD\git\github.com\deejcoder\IdeaNet\ideanet\src-tauri`
- **React Source:** `C:\Users\TonksD\git\github.com\deejcoder\IdeaNet\ideanet\src`

### Common Commands:
```powershell
# Start development server
cd C:\Users\TonksD\git\github.com\deejcoder\IdeaNet\ideanet; yarn tauri dev

# Build for production
cd C:\Users\TonksD\git\github.com\deejcoder\IdeaNet\ideanet; yarn tauri build

# Install dependencies
cd C:\Users\TonksD\git\github.com\deejcoder\IdeaNet\ideanet; yarn

# Android development
cd C:\Users\TonksD\git\github.com\deejcoder\IdeaNet\ideanet; yarn tauri android dev
```

---

*This file serves as a persistent reminder for development best practices and should be referenced in future development sessions.*
