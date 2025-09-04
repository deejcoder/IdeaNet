# Phase 2: Node Focus System (Weeks 5-8)

## Overview
Implement advanced node interaction, focus management, and content expansion systems that make the graph interface intuitive and powerful for knowledge work.

---

## 🎨 **UI-005: Node Focus Management System**
**Priority:** P0 (Critical)  
**Story:** As a user, I want nodes to intelligently focus and expand so that I can work with content contextually without losing track of my place in the graph.

### Acceptance Criteria
- [ ] **Focus State Management**
  - Single focus rule: only one node focused at a time
  - Smooth transitions between focused states
  - Previous focused node returns to minimized state
  - Context preservation during focus changes

- [ ] **Node States Implementation**
  - **Minimized**: Compact view with title/icon only
  - **Focused**: Selected with expanded controls and preview
  - **Expanded**: Full content view with editing capabilities
  - Smooth animations between states

- [ ] **Focus Triggers**
  - Click to focus node
  - Keyboard navigation (arrow keys, tab)
  - Programmatic focus via API calls
  - Focus on node creation

- [ ] **Context Awareness**
  - Connected nodes become highlighted when node is focused
  - Context panel shows related nodes and suggestions
  - Focus affects what's visible in peripheral vision
  - Smart zoom and pan to keep focused content visible

### Technical Requirements
```typescript
interface FocusManager {
  focusedNodeId: string | null;
  expandedNodes: Map<string, string>; // nodeId -> expansionMode
  contextNodes: string[]; // nodes visible due to focus
  
  focusNode(nodeId: string): void;
  expandNode(nodeId: string, mode: string): void;
  collapseNode(nodeId: string): void;
  setContext(nodeIds: string[]): void;
}
```

### Definition of Done
- [ ] Focus system works smoothly with no lag
- [ ] Context awareness enhances navigation
- [ ] Keyboard navigation is intuitive
- [ ] Focus state persists appropriately

---

## 🎨 **UI-006: Node Expansion Modes**
**Priority:** P1 (High)  
**Story:** As a user, I want different ways to view and interact with node content so that I can work efficiently with different types of information.

### Acceptance Criteria
- [ ] **Text Node Expansion**
  - **Preview**: First few lines with formatting preserved
  - **Edit**: Inline rich text editor with toolbar
  - **Fullscreen**: Distraction-free writing mode

- [ ] **Audio Node Expansion**
  - **Play**: Simple play/pause controls with duration
  - **Transcript**: Text view of transcription with timestamps
  - **Waveform**: Visual waveform with chapter markers
  - **Edit**: Transcript editing and chapter management

- [ ] **File Node Expansion**
  - **Preview**: Thumbnail or icon with metadata
  - **Metadata**: Detailed file information and properties
  - **External**: Launch in system default application

- [ ] **Expansion Mode Selection**
  - Smart default expansion based on node type and context
  - User can manually select expansion mode
  - Mode preference remembered per node type
  - Quick mode switching with keyboard shortcuts

### Technical Requirements
```typescript
const NodeExpansionModes = {
  text: ['preview', 'edit', 'fullscreen'],
  audio: ['play', 'transcript', 'waveform', 'edit'],
  file: ['preview', 'metadata', 'external'],
  image: ['thumbnail', 'fullview', 'annotate']
};
```

### Definition of Done
- [ ] All expansion modes work correctly for each node type
- [ ] Mode selection is intuitive and responsive
- [ ] Content displays appropriately in each mode
- [ ] Performance is good with large content

---

## 🔗 **API-006: Enhanced Graph Queries**
**Priority:** P1 (High)  
**Story:** As a developer, I want advanced graph query capabilities so that the frontend can implement smart focus and context features.

### Acceptance Criteria
- [ ] **Graph Traversal API**
  - GET `/api/graph/traverse` - Execute graph traversal queries
  - Support for depth limits and filtering
  - Connection type filtering
  - Node type filtering

- [ ] **Context Queries**
  - Get nodes within N hops of a focused node
  - Find shortest paths between nodes
  - Get strongly connected components
  - Subgraph extraction around focus points

- [ ] **Relationship Analysis**
  - Connection strength calculation
  - Related node discovery
  - Graph clustering and communities
  - Influence and centrality metrics

- [ ] **Query Performance**
  - Efficient graph traversal using SurrealDB's capabilities
  - Query result caching for common patterns
  - Pagination for large result sets

### Technical Requirements
```sql
-- Example traversal queries
SELECT * FROM nodes:start_id <-connects<-nodes<-connects*..3 nodes
WHERE connection_type IN ["reference", "elaborates"];

-- Get context around focused node
SELECT *, count(->connects) AS outgoing, count(<-connects) AS incoming
FROM nodes WHERE id IN (
  SELECT ->connects->nodes FROM nodes:focused_id
  UNION
  SELECT <-connects<-nodes FROM nodes:focused_id
);
```

### Definition of Done
- [ ] Graph traversal queries work efficiently
- [ ] Context queries return relevant results
- [ ] Performance scales with graph complexity
- [ ] Results are properly structured for frontend

---

## 🎨 **UI-007: Keyboard Navigation System**
**Priority:** P1 (High)  
**Story:** As a power user, I want comprehensive keyboard navigation so that I can work efficiently without constantly switching to mouse.

### Acceptance Criteria
- [ ] **Node Navigation**
  - Arrow keys to navigate between connected nodes
  - Tab/Shift+Tab to cycle through nodes in current view
  - Enter to focus/expand current node
  - Escape to collapse and return to overview

- [ ] **Graph Navigation**
  - Space + arrow keys to pan canvas
  - Zoom with +/- or Ctrl+scroll
  - Home to fit graph to view
  - F to focus on selected node

- [ ] **Content Editing**
  - Standard text editing shortcuts (Ctrl+B, Ctrl+I, etc.)
  - Quick node creation (Ctrl+N)
  - Quick connection creation (Ctrl+L)
  - Delete node/connection (Delete key)

- [ ] **Mode Switching**
  - Number keys (1-5) to switch expansion modes
  - M to toggle between focus modes
  - / to focus search
  - ? to show keyboard shortcuts help

### Technical Requirements
```typescript
interface KeyboardManager {
  // Navigation handlers
  navigateToConnectedNode(direction: 'up' | 'down' | 'left' | 'right'): void;
  cycleFocusedNode(direction: 'next' | 'previous'): void;
  
  // Canvas control
  panCanvas(direction: Direction, amount: number): void;
  zoomCanvas(delta: number): void;
  
  // Mode switching
  switchExpansionMode(mode: string): void;
  toggleFocusMode(): void;
}
```

### Definition of Done
- [ ] All keyboard shortcuts work reliably
- [ ] Navigation feels natural and predictable
- [ ] Shortcuts are discoverable and well-documented
- [ ] No conflicts with browser shortcuts

---

## 🎨 **UI-008: Context-Aware Visibility**
**Priority:** P1 (High)  
**Story:** As a user, I want the interface to intelligently show and hide nodes based on what I'm focusing on so that I can concentrate without losing context.

### Acceptance Criteria
- [ ] **Dynamic Visibility**
  - Connected nodes highlighted when node is focused
  - Distant nodes fade out to reduce visual noise
  - Important connections remain visible regardless of distance
  - Smooth opacity transitions

- [ ] **Context Panel**
  - Side panel showing related nodes and connections
  - Quick preview of connected content
  - Suggested actions based on current focus
  - Related content from other projects

- [ ] **Smart Layout**
  - Focused node moves to optimal position
  - Related nodes arrange around focus
  - Layout algorithms respect user positioning
  - Smooth animations during layout changes

- [ ] **Attention Management**
  - Visual cues for unread or updated content
  - Notification system for related activity
  - Focus restoration after interruptions

### Technical Requirements
```typescript
interface ContextManager {
  calculateNodeRelevance(nodeId: string, focusId: string): number;
  updateVisibility(focusId: string): void;
  getRelatedContent(nodeId: string): RelatedItem[];
  suggestActions(context: FocusContext): Action[];
}
```

### Definition of Done
- [ ] Context awareness enhances rather than distracts
- [ ] Visibility changes feel natural and helpful
- [ ] Performance remains smooth with large graphs
- [ ] User can override automatic behavior when needed

---

## 🔗 **API-007: File Metadata and Processing**
**Priority:** P2 (Medium)  
**Story:** As a developer, I want enhanced file processing capabilities so that the frontend can provide rich file experiences.

### Acceptance Criteria
- [ ] **Extended File Metadata**
  - Extract text content from documents
  - Generate thumbnails for images and videos
  - Audio duration and basic analysis
  - File content hashing for deduplication

- [ ] **Content Extraction**
  - PDF text extraction for search
  - Image EXIF data processing
  - Audio metadata (artist, duration, etc.)
  - Document structure analysis

- [ ] **File Processing Pipeline**
  - Background processing for uploaded files
  - Processing status tracking
  - Error handling for unsupported formats
  - Retry mechanisms for failed processing

### Definition of Done
- [ ] File metadata is automatically extracted
- [ ] Content is available for search and analysis
- [ ] Processing happens efficiently in background
- [ ] All file types are handled gracefully

---

## 🎨 **UI-009: Enhanced Node Types**
**Priority:** P2 (Medium)  
**Story:** As a user, I want rich node types with appropriate expansion modes so that I can work with different content types effectively.

### Acceptance Criteria
- [ ] **Rich Text Nodes**
  - Markdown support with live preview
  - Block-style editing (headings, lists, quotes)
  - Link creation to other nodes
  - Tag and category system

- [ ] **Enhanced File Nodes**
  - Image nodes with zoom and annotation
  - Document nodes with page navigation
  - Video nodes with playback controls
  - Code nodes with syntax highlighting

- [ ] **Specialized Content**
  - URL/bookmark nodes with preview
  - Task/todo nodes with completion tracking
  - Date/event nodes with calendar integration
  - Location nodes with map display

### Definition of Done
- [ ] Each node type provides appropriate functionality
- [ ] Expansion modes are logical and useful
- [ ] Content editing is smooth and intuitive
- [ ] Visual design is consistent across types

---

## 🎨 **UI-010: Advanced Canvas Features**
**Priority:** P2 (Medium)  
**Story:** As a user, I want advanced canvas features so that I can organize and navigate large knowledge graphs effectively.

### Acceptance Criteria
- [ ] **Canvas Organization**
  - Multiple zoom levels with appropriate detail
  - Minimap for large graph navigation
  - Canvas regions/areas for organization
  - Bookmarked viewpoints

- [ ] **Visual Enhancements**
  - Connection styling based on type and strength
  - Node clustering and grouping
  - Visual themes and customization
  - Grid and alignment helpers

- [ ] **Performance Optimization**
  - Level-of-detail rendering for large graphs
  - Virtualization for thousands of nodes
  - Smooth animations and transitions
  - Efficient canvas updates

### Definition of Done
- [ ] Canvas handles large graphs smoothly
- [ ] Visual organization aids understanding
- [ ] Performance remains good with 1000+ nodes
- [ ] User can customize appearance to preferences

---

## 📋 **Phase 2 Deliverables**

### **Week 5-6: Focus System**
- Node focus management with smooth transitions
- Context-aware visibility and highlighting
- Keyboard navigation system

### **Week 7-8: Expansion & Polish**
- Multiple expansion modes for different content types
- Enhanced file processing and metadata
- Advanced canvas features for large graphs

### **Success Criteria**
- [ ] Graph interaction feels natural and responsive
- [ ] Users can focus on content without losing context
- [ ] Keyboard navigation enables efficient workflow
- [ ] System scales gracefully with graph complexity
- [ ] Content types have appropriate interaction modes

### **Key Metrics**
- Focus transition time < 200ms
- Keyboard shortcuts have 95%+ reliability
- Canvas performance >30fps with 500+ nodes
- User can navigate graph entirely via keyboard

### **Technical Improvements**
- Implement proper state management for focus
- Add performance monitoring and optimization
- Create comprehensive keyboard shortcut system
- Build foundation for plugin system integration
