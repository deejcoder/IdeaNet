# Audio Module PRD (`ideanet-audio`)

## Overview
The audio module provides comprehensive audio recording, processing, and transcription capabilities for natural idea capture. It handles microphone and system audio recording, AI-powered transcription with word-level timestamps, and intelligent audio content integration.

## Technical Architecture

### Core Components
- **AudioRecorder**: Multi-source audio capture with cpal and WASAPI
- **TranscriptionService**: OpenAI Whisper integration with segmentation
- **AudioProcessor**: Signal processing and quality enhancement
- **PlaybackManager**: Synchronized audio playback with transcript highlighting

### Dependencies
```toml
[dependencies]
# Audio processing
cpal = "0.15"           # Cross-platform audio I/O
hound = "3.5"           # WAV file encoding/decoding
symphonia = "0.5"       # Audio decoding for segmentation
dasp = "0.11"           # Digital audio signal processing

# Windows-specific audio
[target.'cfg(windows)'.dependencies]
wasapi = "0.19"         # Windows audio session API

# AI and HTTP
reqwest = { version = "0.12", features = ["json", "multipart"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.0", features = ["v4"] }
```

## User Stories

### Audio Capture

### 8.1 High-quality microphone recording
- **ID**: US-AUDIO-001
- **Description**: As a user, I want to record clear audio from my microphone so that my spoken ideas are captured with high fidelity
- **Acceptance criteria**:
  - Enumerate and select from available microphone devices
  - Real-time audio level monitoring with visual indicators
  - Configurable sample rate (44.1kHz, 48kHz) and bit depth (16-bit, 24-bit)
  - Background noise reduction and automatic gain control
  - Recording quality presets (voice, music, high-fidelity)

### 8.2 System audio loopback recording
- **ID**: US-AUDIO-002
- **Description**: As a user, I want to record system audio output so that I can capture meeting audio, videos, and other computer sounds
- **Acceptance criteria**:
  - Windows WASAPI loopback endpoint access for system audio
  - Simultaneous microphone and system audio recording to separate tracks
  - Audio mixing options for combined or separate output files
  - Volume level control for each audio source
  - Real-time monitoring without audio feedback loops

### 8.3 Recording session management
- **ID**: US-AUDIO-003
- **Description**: As a user, I want to manage recording sessions effectively so that I can organize and access my audio content efficiently
- **Acceptance criteria**:
  - Unique session IDs with timestamp-based file naming
  - Recording metadata capture (duration, sources, quality settings)
  - Automatic file organization by project and date
  - Recording history with playback and transcription status
  - Bulk operations for managing multiple recordings

### Audio Processing and Transcription

### 8.4 Intelligent audio segmentation
- **ID**: US-AUDIO-004
- **Description**: As a developer, I want to handle large audio files automatically so that API limits don't prevent transcription of long recordings
- **Acceptance criteria**:
  - Automatic detection of files exceeding API size limits (25MB)
  - Intelligent segmentation avoiding cuts during speech
  - Overlap regions between segments for continuity
  - Segment metadata tracking for reassembly
  - Progress reporting for multi-segment processing

### 8.5 High-accuracy transcription
- **ID**: US-AUDIO-005
- **Description**: As a user, I want my audio automatically transcribed with high accuracy so that spoken content becomes searchable text
- **Acceptance criteria**:
  - OpenAI Whisper integration with API key management
  - Word-level timestamp extraction for precise navigation
  - Confidence scoring for transcription quality assessment
  - Custom vocabulary support for domain-specific terms
  - Fallback to alternative transcription services if primary fails

### 8.6 Speaker identification and diarization
- **ID**: US-AUDIO-006
- **Description**: As a user, I want to identify different speakers in multi-person recordings so that I can understand who said what
- **Acceptance criteria**:
  - Automatic speaker detection and labeling
  - Speaker timeline visualization in transcription
  - Manual speaker identification and correction
  - Speaker voice fingerprinting for future recognition
  - Export options preserving speaker information

### Audio Playback and Navigation

### 8.7 Synchronized transcript playback
- **ID**: US-AUDIO-007
- **Description**: As a user, I want to navigate audio using the transcript so that I can quickly find and replay specific moments
- **Acceptance criteria**:
  - Click any word to jump to corresponding audio timestamp
  - Real-time highlighting of current word during playback
  - Smooth seeking without audio artifacts or delays
  - Playback speed control (0.25x to 3x) maintaining pitch
  - Keyboard shortcuts for common playback operations

### 8.8 Advanced playback features
- **ID**: US-AUDIO-008
- **Description**: As a user, I want sophisticated playback controls so that I can analyze and review audio content effectively
- **Acceptance criteria**:
  - A-B repeat functionality for focused listening
  - Audio bookmarking with named markers
  - Loop playback for specific segments or sentences
  - Skip silence detection and automatic removal
  - Audio visualization with waveform display

### 8.9 Audio content search
- **ID**: US-AUDIO-009
- **Description**: As a user, I want to search within audio transcriptions so that I can find specific topics or quotes quickly
- **Acceptance criteria**:
  - Full-text search across all transcriptions
  - Search result highlighting with context snippets
  - Time-based filtering (find mentions in specific time ranges)
  - Semantic search for conceptually similar content
  - Search within specific recordings or across entire project

### Integration and Export

### 8.10 Block-based audio integration
- **ID**: US-AUDIO-010
- **Description**: As a user, I want to embed audio recordings in my notes so that spoken ideas integrate with written content
- **Acceptance criteria**:
  - Audio block plugin with embedded player controls
  - Transcript display within the block with interactive navigation
  - Audio segment extraction and embedding (start/end timestamps)
  - Link audio clips to related text content and ideas
  - Collaborative commenting on specific audio timestamps

### 8.11 Audio content analysis
- **ID**: US-AUDIO-011
- **Description**: As a user, I want AI analysis of my audio content so that I can extract insights and action items automatically
- **Acceptance criteria**:
  - Automatic summary generation from long recordings
  - Key topic extraction and tagging
  - Action item and task identification
  - Sentiment analysis and emotional tone detection
  - Meeting transcript formatting with agenda items

### 8.12 Audio export and sharing
- **ID**: US-AUDIO-012
- **Description**: As a user, I want to export audio content in various formats so that I can share and use it in other applications
- **Acceptance criteria**:
  - Multiple audio format export (WAV, MP3, M4A, FLAC)
  - Transcript export with timestamps (SRT, VTT, JSON)
  - Audio segment extraction with metadata preservation
  - Podcast-style export with chapters and show notes
  - Integration with external transcription services

## Technical Implementation Details

### Audio Recording Pipeline
```rust
pub struct AudioRecorder {
    recording_id: Option<String>,
    microphone_writer: Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>,
    microphone_stream: Option<cpal::Stream>,
    system_audio_recorder: Option<SystemAudioRecorder>,
    is_recording: bool,
    quality_settings: AudioQuality,
}

impl AudioRecorder {
    pub async fn start_recording(&mut self, options: RecordingOptions) -> Result<String> {
        // 1. Initialize audio devices and streams
        // 2. Create WAV writers for each source
        // 3. Start recording threads
        // 4. Return session ID
    }
    
    pub async fn stop_recording(&mut self) -> Result<Vec<AudioFile>> {
        // 1. Stop all recording streams
        // 2. Finalize WAV files
        // 3. Generate metadata
        // 4. Return file paths and metadata
    }
}
```

### Transcription Service
```rust
pub struct TranscriptionService {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
}

impl TranscriptionService {
    pub async fn transcribe_detailed(&self, audio_path: &Path) -> Result<DetailedTranscription> {
        // 1. Check file size and segment if needed
        // 2. Upload to OpenAI Whisper API
        // 3. Extract word-level timestamps
        // 4. Process speaker diarization if multi-speaker
        // 5. Return structured transcription data
    }
}

#[derive(Serialize, Deserialize)]
pub struct DetailedTranscription {
    pub text: String,
    pub words: Vec<TranscriptionWord>,
    pub segments: Vec<TranscriptionSegment>,
    pub speakers: Option<Vec<Speaker>>,
    pub confidence: f32,
    pub language: String,
    pub duration: f64,
}
```

### Audio Block Plugin Interface
```typescript
interface AudioBlockProps extends BlockProps {
  audioPath: string;
  transcription?: DetailedTranscription;
  playbackState: PlaybackState;
  onTimestampClick: (timestamp: number) => void;
  onSegmentSelect: (start: number, end: number) => void;
}

export const AudioBlock: React.FC<AudioBlockProps> = ({
  audioPath,
  transcription,
  playbackState,
  onTimestampClick,
  onSegmentSelect
}) => {
  // Render audio player with synchronized transcript
  // Handle word-level navigation and highlighting
  // Provide playback controls and audio visualization
};
```

## Performance Requirements

### Recording Performance
- **Latency**: < 20ms for real-time monitoring
- **Dropout Prevention**: 99.9% uptime during recording sessions
- **Multi-source Sync**: < 1ms timing difference between sources
- **Resource Usage**: < 50MB RAM per recording session

### Transcription Performance
- **API Response Time**: < 30 seconds for 5-minute audio files
- **Accuracy Target**: > 95% word accuracy for clear speech
- **Segmentation Efficiency**: < 2% overhead for large file processing
- **Concurrent Requests**: Support 3+ simultaneous transcription jobs

### Playback Performance
- **Seek Accuracy**: < 50ms timing precision for word navigation
- **Loading Time**: < 2 seconds for audio file initialization
- **Memory Efficiency**: Stream large files without full loading
- **UI Responsiveness**: < 100ms for playback control responses

## Testing Strategy

### Unit Tests
- Audio device enumeration and configuration
- WAV file writing and format validation
- Transcription API integration and error handling
- Audio segmentation accuracy and timing

### Integration Tests
- End-to-end recording and transcription workflow
- Multi-source audio synchronization
- Block integration with frontend components
- Cross-platform audio compatibility

### Performance Tests
- Long recording session stability (2+ hours)
- Large file transcription (100MB+ audio files)
- Concurrent recording and playback operations
- Memory usage under various workloads
