# IdeaNet - Product Requirements Document

## 1. Overv### Core Modules:
1. **`ideanet-core`** - Core domain models, business logic, and database integration
2. **`ideanet-audio`** - Audio recording, processing, and transcription services
3. **`ideanet-blocks`** - Plugin-based block system for extensible content types
4. **`ideanet-ai`** - AI integration for suggestions, research, and content generation
5. **`ideanet-resources`** - File and resource management with linking strategies
6. **`ideanet-ui`** - React frontend with Chakra UI components

*Note: Each module contains its own subset of user stories detailed in separate module PRDs. Modules should be implemented independently while maintaining clear interfaces between them.*deaNet is an AI-powered idea crafting and project development platform that helps users express, organize, and evolve their ideas into actionable implementation plans. Built as a desktop application using Tauri, React, and TypeScript, it provides intelligent project management with embedded resources, AI-guided development suggestions, and rich multimedia capture including audio recording and transcription.

## 2. Executive Summary

IdeaNet transforms how creators, entrepreneurs, and teams develop ideas from initial conception to implementation. By combining rich multimedia capture (including audio recording with AI transcription), flexible block-based content creation, intelligent resource management, and AI-powered guidance, users can build comprehensive project documentation that evolves with their thinking.

### Key Differentiators
- **Plugin-based Block System**: Extensible content blocks for different media types and workflows
- **Audio-First Capture**: High-quality audio recording with AI transcription for natural idea capture
- **AI Development Assistant**: Context-aware suggestions for next steps and implementation guidance
- **Smart Resource Linking**: Flexible file management with soft/deep linking strategies
- **Project Evolution Tracking**: Visual timeline of how ideas develop and change over time

## 3. Narrative

Sarah, a product manager, has a breakthrough idea during her morning commute. She opens IdeaNet and quickly records a 3-minute audio note explaining her concept. The AI automatically transcribes it and suggests related market research questions. Back at her desk, she creates a new project, embeds the transcription into a structured idea block, and links relevant competitor analysis documents. 

As she develops the concept over the following weeks, IdeaNet's AI suggests implementation frameworks, identifies potential technical challenges, and recommends team members based on similar past projects. The block-based system lets her organize thoughts naturally - from audio brainstorms to technical specifications to market analysis - all interconnected and searchable.

When presenting to stakeholders, she exports a clean presentation view that shows the idea's evolution from initial audio capture to comprehensive business plan, demonstrating both the depth of thinking and the structured development process.

## 4. User Goals

### Primary Goals
- **Capture ideas naturally** without losing spontaneous insights
- **Develop ideas systematically** with AI guidance and structured thinking
- **Connect related concepts** across projects and time periods
- **Create implementation roadmaps** that turn ideas into actionable plans
- **Collaborate effectively** by sharing rich, multimedia project documentation

### Secondary Goals
- **Track idea evolution** to understand and improve thinking processes
- **Maintain project context** through rich resource linking and organization
- **Access information quickly** through powerful search and AI assistance
- **Present ideas professionally** with clean export and presentation modes

## 5. Business Goals

### Product Goals
- **Create differentiated IP platform** in the idea management space
- **Build sustainable competitive moat** through AI integration and user data
- **Establish platform ecosystem** for third-party block plugins and integrations
- **Generate recurring revenue** through AI service usage and premium features

### Market Goals
- **Capture knowledge worker market** (product managers, designers, researchers, entrepreneurs)
- **Expand to enterprise teams** with collaboration and administration features
- **Partner with AI providers** to offer specialized domain knowledge
- **Build community ecosystem** around plugin development and templates

## 6. System Architecture

### Core Modules
1. **`ideanet-core`** - Core domain models and business logic
2. **`ideanet-audio`** - Audio recording, processing, and transcription services
3. **`ideanet-blocks`** - Plugin-based block system for extensible content types
4. **`ideanet-ai`** - AI integration for suggestions, research, and content generation
5. **`ideanet-resources`** - File and resource management with linking strategies
6. **`ideanet-db`** - Database layer with SurrealDB embedded storage
7. **`ideanet-ui`** - React frontend with Chakra UI components

### Plugin Architecture
Blocks are designed as independent plugins that can be developed separately and loaded dynamically:

```typescript
interface BlockPlugin {
  id: string;
  name: string;
  version: string;
  component: React.ComponentType<BlockProps>;
  schema: BlockSchema;
  config?: BlockConfig;
}
```

---

## 7. User Stories (US-XXX)

*Note: Each module has its own detailed PRD with additional user stories:*
- *Audio Module: See [Audio Module PRD](./modules/audio.md) for US-AUDIO-001 through US-AUDIO-012*
- *Blocks Module: See [Blocks Module PRD](./modules/blocks.md) for US-BLOCKS-001 through US-BLOCKS-014*
- *AI Services Module: See [AI Services PRD](./modules/ai-services.md) for US-AI-001 through US-AI-015*
- *Resources Module: See [Resources Module PRD](./modules/resources.md) for US-RESOURCES-001 through US-RESOURCES-008*

### 7.1 Project Foundation and Setup (Core Module)

### 7.1.1 Initialize new IdeaNet project
- **ID**: US-001
- **Description**: As a developer, I want to set up a new Tauri project with React frontend so that I can start building the application from a solid foundation
- **Acceptance criteria**:
  - Tauri project scaffolding with React + TypeScript frontend
  - Windows (WinUI) and Android platform configuration
  - Basic development environment with hot reload and debugging tools
  - Project structure following DDD architecture principles

### 7.1.2 Establish database connection foundation
- **ID**: US-001.1
- **Description**: As a developer, I want to establish a basic database connection layer so that the application can reliably connect to and communicate with SurrealDB
- **Acceptance criteria**:
  - SurrealDB embedded database integration with RocksDB backend
  - Database connection management (connect/disconnect operations)
  - Health check functionality to verify database status
  - Proper error handling for connection failures
  - Database configuration management for different platforms

### 7.1.3 Implement clean architecture foundation
- **ID**: US-001.2  
- **Description**: As a developer, I want to implement DDD architecture layers so that the codebase is maintainable and supports future modular expansion
- **Acceptance criteria**:
  - Domain layer with error types and core abstractions
  - Application layer with service interfaces (ports) and business logic
  - Infrastructure layer with concrete database implementations
  - Tauri commands layer for frontend-backend communication
  - Clear separation of concerns following hexagonal architecture

### 7.1.4 Create database connection UI
- **ID**: US-001.3
- **Description**: As a developer, I want a simple UI to test database connectivity so that I can verify the foundation is working correctly
- **Acceptance criteria**:
  - React component displaying real-time database connection status
  - Connect/disconnect buttons for manual connection testing
  - Health check functionality with status indicators
  - Error message display for connection failures
  - Basic responsive design for different screen sizes

### 7.1.5 Establish future-ready architecture
- **ID**: US-001.4
- **Description**: As a developer, I want the architecture to support future plugin systems and multi-crate modules so that the foundation can scale without major refactoring
- **Acceptance criteria**:
  - Code structure that supports future event bus implementation
  - Module boundaries designed for independent crate development
  - Interface definitions that can accommodate future audio, AI, and block modules
  - Configuration system ready for multi-module setup
  - Documentation of architectural decisions and extension points

**Note**: User stories US-001.1 through US-001.4 represent the minimal scope for Phase 1 foundation implementation. All other features (business entities, CRUD operations, plugin system, event bus, testing framework) are explicitly deferred to future phases.

### 7.1.6 Configure database foundation (Future - Phase 2)
- **ID**: US-002
- **Description**: As a developer, I want SurrealDB properly configured with schema and migrations so that data persistence works reliably across all modules
- **Acceptance criteria**:
  - Database schema migration system for version updates  
  - Database connection pooling and error recovery
  - Data integrity constraints and validation rules
  - Business entity definitions and CRUD operations
  - Performance optimization and indexing strategies

### 7.1.7 Establish module interfaces (Future - Phase 3)
- **ID**: US-003
- **Description**: As a developer, I want clear interfaces between modules so that they can be developed independently while maintaining system coherence
- **Acceptance criteria**:
  - Event bus implementation for loose coupling between modules
  - Plugin registry system for dynamic module loading
  - Error handling patterns consistent across all modules
  - Module initialization and dependency injection system
  - Integration test framework for module interactions

### 7.1.8 Integrate UI framework foundation (Future - Phase 2)
- **ID**: US-002.1
- **Description**: As a developer, I want to integrate Chakra UI component library so that the application has consistent, accessible styling from the beginning
- **Acceptance criteria**:
  - Chakra UI component library setup with consistent theming
  - Dark/light mode theme system with user preference persistence
  - Accessible UI components following WCAG guidelines
  - Theme customization for IdeaNet branding
  - Responsive design foundation for different screen sizes

### 7.2 Project Management (Future - Phase 3)

**Note**: All project management features are deferred to Phase 3. The foundation implementation (Phase 1) focuses solely on database connectivity and basic architecture.

### 7.2.1 Create and organize projects (Future - Phase 3)
- **ID**: US-004
- **Description**: As a user, I want to create projects with descriptive metadata so that I can organize my ideas around specific goals and themes
- **Acceptance criteria**:
  - User can create project with required name and optional description
  - Projects support tagging for cross-cutting organization
  - Project status tracking (active, archived, completed, on-hold)
  - Project templates available for common use cases
  - Duplicate project detection and merging suggestions

### 7.2.2 Navigate project hierarchy (Future - Phase 3)
- **ID**: US-005
- **Description**: As a user, I want to see my project structure clearly so that I can understand relationships and quickly access relevant content
- **Acceptance criteria**:
  - Visual project dashboard with statistics and recent activity
  - Hierarchical navigation with breadcrumbs
  - Quick access sidebar with pinned projects and favorites
  - Project search with metadata filtering
  - Recently accessed projects prominently displayed

### 7.2.3 Manage project lifecycle (Future - Phase 3)
- **ID**: US-006
- **Description**: As a user, I want to track project progress and lifecycle stages so that I can maintain focus and measure advancement
- **Acceptance criteria**:
  - Project status workflow (concept → development → testing → complete)
  - Progress indicators based on completed milestones
  - Timeline view showing project evolution over time
  - Archive/restore functionality for completed projects
  - Bulk operations for project management

### 7.3 Audio Recording and Transcription (Audio Module)
*See [Audio Module PRD](./modules/audio.md) for detailed technical implementation*

### 7.3.1 Capture high-quality audio
- **ID**: US-007
- **Description**: As a user, I want to record clear audio from multiple sources so that I can capture ideas naturally through speech
- **Acceptance criteria**:
  - Microphone recording with device selection and level monitoring
  - System audio recording for capturing meeting audio or media
  - Simultaneous multi-source recording with separate tracks
  - Real-time audio level indicators and quality feedback
  - Background noise reduction and audio enhancement options

### 7.3.2 Process audio intelligently
- **ID**: US-008
- **Description**: As a user, I want my audio recordings automatically processed and transcribed so that they become searchable and actionable content
- **Acceptance criteria**:
  - Automatic transcription using OpenAI Whisper with word-level timestamps
  - Speaker diarization for multi-person recordings
  - Audio segmentation for large files to handle API limits
  - Confidence scoring for transcription accuracy
  - Manual transcription correction with learning feedback

### 7.3.3 Navigate audio content
- **ID**: US-009
- **Description**: As a user, I want to interact with audio recordings through their transcriptions so that I can quickly find and reference specific moments
- **Acceptance criteria**:
  - Click words in transcription to jump to audio timestamp
  - Visual highlighting of current word during playback
  - Playback speed control (0.5x to 2x speed)
  - A-B repeat functionality for focused listening
  - Audio bookmarking for important segments

### 7.3.4 Integrate audio with other content
- **ID**: US-010
- **Description**: As a user, I want to embed audio recordings and transcriptions into my project documentation so that spoken ideas become part of my structured thinking
- **Acceptance criteria**:
  - Audio block plugin with embedded player and transcription
  - Export transcription text to other blocks or external formats
  - Link audio segments to related notes and resources
  - Audio summary generation using AI analysis
  - Collaborative commenting on specific audio timestamps

### 7.3.5 Integrate rich text editor (Future - Phase 4)
- **ID**: US-010.1
- **Description**: As a developer, I want to integrate BlockNote.js rich text editor so that users can create structured, extensible content blocks
- **Acceptance criteria**:
  - BlockNote.js integration with custom block support and extensibility
  - Custom block plugins foundation for future extensible content types
  - Integration between BlockNote editor and Chakra UI design system
  - Block serialization and deserialization for database storage
  - Editor toolbar customization for IdeaNet-specific features

### 7.4 Block System and Content Creation (Blocks Module)
*See [Blocks Module PRD](./modules/blocks.md) for detailed plugin architecture*

### 7.4.1 Create structured content blocks
- **ID**: US-011
- **Description**: As a user, I want to organize my ideas using different content block types so that I can structure information appropriately for different purposes
- **Acceptance criteria**:
  - Idea block with title, description, priority, and status tracking
  - Implementation plan block with steps, timeline, and dependencies
  - Research block with questions, sources, and findings
  - Decision block with options, criteria, and rationale
  - Reference block for external sources with metadata

### 7.4.2 Customize block behavior
- **ID**: US-012
- **Description**: As a user, I want to configure how blocks behave and display so that they match my workflow preferences
- **Acceptance criteria**:
  - Block templates with pre-configured fields and layouts
  - Custom field addition for specialized use cases
  - Block styling and theme customization
  - Conditional logic for showing/hiding fields based on content
  - Block validation rules for required fields and data quality

### 7.4.3 Link and reference blocks
- **ID**: US-013
- **Description**: As a user, I want to create connections between blocks so that I can build a network of related ideas and information
- **Acceptance criteria**:
  - Bidirectional linking between blocks with relationship types
  - Visual graph view of block connections
  - Backlinks panel showing incoming references
  - Link suggestions based on content similarity
  - Bulk link operations for connecting related content

### 7.4.4 Extend functionality with plugins
- **ID**: US-014
- **Description**: As a developer, I want to create custom block types so that I can extend IdeaNet's functionality for specialized use cases
- **Acceptance criteria**:
  - Plugin development SDK with TypeScript definitions
  - Block registration and lifecycle management
  - Plugin store for discovering and installing extensions
  - Sandbox security model for third-party plugins
  - Plugin versioning and compatibility management

### 7.5 AI-Powered Assistance (AI Services Module)
*See [AI Services Module PRD](./modules/ai-services.md) for detailed AI integration*

### 7.5.1 Generate contextual suggestions
- **ID**: US-015
- **Description**: As a user, I want to receive AI suggestions based on my current content so that I can discover new directions and overcome creative blocks
- **Acceptance criteria**:
  - Context-aware suggestions based on selected text or blocks
  - Multiple suggestion types (next steps, related ideas, questions, critiques)
  - Confidence scoring and ranking of suggestions
  - Suggestion history and ability to revisit previous ideas
  - User feedback loop to improve suggestion quality

### 7.5.2 Research and fact-checking assistance
- **ID**: US-016
- **Description**: As a user, I want AI help with research and verification so that I can build well-informed and credible projects
- **Acceptance criteria**:
  - Generate research questions for selected topics
  - Fact-checking with source citations and confidence levels
  - Alternative perspective generation for balanced analysis
  - Trend analysis and market research suggestions
  - Expert opinion synthesis from multiple sources

### 7.5.3 Create implementation roadmaps
- **ID**: US-017
- **Description**: As a user, I want AI assistance in turning abstract ideas into concrete action plans so that I can move from concept to execution
- **Acceptance criteria**:
  - Step-by-step implementation plan generation
  - Technology stack recommendations with rationale
  - Risk assessment and mitigation strategies
  - Timeline estimation with milestone planning
  - Resource requirement analysis and team skill mapping

### 7.5.4 Enhance content quality
- **ID**: US-018
- **Description**: As a user, I want AI help improving my writing and documentation so that I can communicate ideas more effectively
- **Acceptance criteria**:
  - Grammar and style improvement suggestions
  - Clarity and structure recommendations
  - Audience-appropriate tone and language adjustments
  - Summary generation for complex content
  - Presentation outline creation from project content

### 7.6 Resource Management (Resources Module)
*See [Resources Module PRD](./modules/resources.md) for detailed resource handling*

### 7.6.1 Link external resources flexibly
- **ID**: US-019
- **Description**: As a user, I want to connect external files and URLs to my projects so that I can maintain comprehensive context without duplicating content
- **Acceptance criteria**:
  - Soft linking with broken link detection and notifications
  - Deep copying for critical resources with version management
  - URL metadata extraction (title, description, preview image)
  - File type detection and appropriate preview generation
  - Batch import operations for multiple resources

### 7.6.2 Organize and search resources
- **ID**: US-020
- **Description**: As a user, I want to find and organize my project resources efficiently so that I can access relevant information quickly
- **Acceptance criteria**:
  - Resource categorization with tags and custom taxonomies
  - Full-text search across resource content and metadata
  - Visual thumbnail grid for images and documents
  - Resource usage tracking and related content suggestions
  - Duplicate detection and consolidation recommendations

### 7.6.3 Embed rich media content
- **ID**: US-021
- **Description**: As a user, I want to embed various media types directly in my notes so that I can create rich, multimedia documentation
- **Acceptance criteria**:
  - Image embedding with annotation and captioning
  - Video embedding with timestamp linking
  - PDF viewer with highlight and note capabilities
  - Interactive charts and data visualizations
  - Code snippet execution and syntax highlighting

### 7.6.4 Manage storage efficiently
- **ID**: US-022
- **Description**: As a user, I want to understand and control my storage usage so that I can maintain optimal performance and cost
- **Acceptance criteria**:
  - Storage usage dashboard with breakdown by project and resource type
  - Compression options for large files
  - Archive/cleanup suggestions for unused resources
  - Storage quota warnings and management tools
  - Export/backup options for data portability

### 7.7 Search and Discovery (Core Module)

### 7.7.1 Search across all content
- **ID**: US-023
- **Description**: As a user, I want to search across all my projects and content types so that I can find relevant information regardless of where it's stored
- **Acceptance criteria**:
  - Global search across projects, notes, resources, and transcriptions
  - Search result ranking by relevance, recency, and user interaction patterns
  - Advanced filtering by content type, date range, tags, and project
  - Search suggestions and autocomplete based on content and history
  - Saved searches and search alerts for new matching content

### 7.7.2 Discover content relationships
- **ID**: US-024
- **Description**: As a user, I want to discover connections between my content so that I can identify patterns and build on previous work
- **Acceptance criteria**:
  - Semantic similarity search using AI embeddings
  - "Find similar" functionality for any content item
  - Content clustering and theme identification
  - Timeline view showing content evolution and relationships
  - Recommendation engine for related projects and ideas

### 7.7.3 Handle empty states gracefully
- **ID**: US-025
- **Description**: As a user, I want to see helpful guidance when areas are empty so that I understand how to get started and make progress
- **Acceptance criteria**:
  - Welcome flow for new users with guided project creation
  - Empty project states with suggestions for first steps
  - Empty search results with alternative query suggestions
  - Onboarding tooltips and progressive disclosure of features
  - Template suggestions based on user goals and project type

### 7.8 Export and Sharing (Core Module)

### 7.8.1 Export projects in multiple formats
- **ID**: US-026
- **Description**: As a user, I want to export my projects in various formats so that I can share them appropriately with different audiences
- **Acceptance criteria**:
  - PDF export with embedded media and clickable links
  - Markdown export with linked resources and standard formatting
  - HTML export for web publishing with responsive design
  - Presentation mode with slides generated from project structure
  - JSON export for data portability and integration

### 7.8.2 Create shareable project views
- **ID**: US-027
- **Description**: As a user, I want to create focused views of my projects so that I can share relevant information without exposing sensitive details
- **Acceptance criteria**:
  - Custom view creation with filtered content
  - Permission-based sharing with read-only access
  - Public link generation with expiration dates
  - Embedded view widgets for external websites
  - Analytics on shared content engagement

### 7.9 Collaboration Features (Core Module)

### 7.9.1 Comment and annotate content
- **ID**: US-028
- **Description**: As a collaborator, I want to add comments and annotations to project content so that I can provide feedback and suggestions
- **Acceptance criteria**:
  - Inline comments on text selections and blocks
  - Annotation tools for images and documents
  - Comment threading and resolution tracking
  - @mention notifications for team members
  - Comment export and archive capabilities

### 7.9.2 Track changes and versions
- **ID**: US-029
- **Description**: As a user, I want to see how content has changed over time so that I can understand the evolution of ideas and revert if needed
- **Acceptance criteria**:
  - Automatic version history for all content changes
  - Visual diff view showing additions, deletions, and modifications
  - Version comparison between any two points in time
  - Selective revert functionality for specific changes
  - Change attribution and timestamping

---

## 8. Technical Implementation Priorities

### Phase 1: Foundation Only (Current Scope - Weeks 1-2)
- **US-001**: Tauri project setup with React + TypeScript frontend
- **US-001.1**: Database connection foundation with SurrealDB
- **US-001.2**: Clean DDD architecture implementation  
- **US-001.3**: Basic database connection UI for testing
- **US-001.4**: Future-ready architecture design
- **Deliverable**: Working database connectivity with health checking UI

**Explicitly Out of Scope for Phase 1:**
- ❌ Business entities, CRUD operations, schema migrations
- ❌ Plugin system, event bus, module interfaces  
- ❌ Audio, AI, blocks, or resource management features
- ❌ Unit/integration testing (deferred to Phase 2)
- ❌ BlockNote.js, Chakra UI integration

### Phase 2: Testing & Core Entities (Future - Weeks 3-6)
- **US-002**: Database schema, migrations, and entity persistence
- **US-002.1**: Chakra UI component library integration and theming
- **Testing framework**: Unit and integration test implementation
- **US-004, US-005, US-006**: Basic project management features
- **Deliverable**: Reliable data persistence with project CRUD operations and consistent UI

### Phase 3: Module Architecture (Future - Weeks 7-10)  
- **US-003**: Event bus and module interface implementation
- **US-019, US-020**: Resource linking and organization
- **US-023**: Basic search functionality
- **Deliverable**: Modular architecture ready for plugin development

### Phase 4: Content Creation & Block System (Future - Weeks 11-14)
- **US-010.1**: BlockNote.js rich text editor integration
- **US-011, US-012**: Basic block system and customization
- **Deliverable**: Rich content creation with extensible block system

### Phase 5: Audio Integration (Future - Weeks 15-18)
- **US-007, US-008**: Audio recording and transcription
- **US-009, US-010**: Audio navigation and integration
- **Audio module**: Complete implementation with Whisper integration
- **Deliverable**: Full audio-to-text workflow

### Phase 6: AI Integration (Future - Weeks 19-22)
- **US-015, US-016**: AI suggestions and research assistance
- **US-017, US-018**: Implementation planning and content enhancement
- **AI service**: Architecture and context-aware assistance
- **Deliverable**: Intelligent content suggestions and analysis

### Phase 7: Advanced Features (Future - Weeks 23-26)
- **US-013, US-014**: Block linking and plugin system
- **US-021, US-022**: Rich media and storage management
- **US-024, US-025**: Content discovery and empty states
- **Deliverable**: Full plugin ecosystem and advanced UI

### Phase 8: Collaboration and Polish (Future - Weeks 27-30)
- **US-026, US-027**: Export and sharing capabilities
- **US-028, US-029**: Collaboration features
- **Performance optimization**: User experience polish
- **Deliverable**: Production-ready application with collaboration
