# Fastnote – File Structure Specification

## Projects, Folders, Files

Fastnote uses a hierarchical structure entirely based on Linux folders:

- A **Fastnote Project** is a folder.  
- A **Fastnote Folder** is a folder inside a project.  
- A **Fastnote File** is also a folder.

Each of these contains a `manifest.json` describing its type.

### Project

A project folder contains:

- `manifest.json` → `{ "type": "project", ... }` with project metadata (color, name, etc.)
- Fastnote folders  
- Fastnote files  

A project can be stored anywhere on the filesystem. (Where you can r/w) 

### Folder

A folder contains:

- `manifest.json` → `{ "type": "folder" }`
- Other Fastnote folders  
- Fastnote files  

### File

A file contains:

- `manifest.json` → `{ "type": "file" }`
- One or more Fastnote pages

---

## Pages, Regions, Chunks

### Page

A Fastnote Page is also a folder.

It must contain:

manifest.json → { "type": "page" }
regions/      → folder containing region files
image/        → folder containing CPU-rendered images

### Region

A region is a Linux file.

- It stores a **4×4 grid of chunks**.
- It must be named: r_{x}_{y}.json

### Chunk

A chunk is a 2000px × 1000px area containing strokes.

- Stroke rendering is GPU-accelerated.  
- Image rendering is CPU-generated.  


AI generated example:

FastnoteProject/
├── manifest.json                # { "type": "project", ... }
├── Math/                        # Fastnote Folder
│   ├── manifest.json            # { "type": "folder" }
│   ├── Limits/                  # Fastnote File
│   │   ├── manifest.json        # { "type": "file" }
│   │   ├── Page_1/              # Fastnote Page
│   │   │   ├── manifest.json    # { "type": "page" }
│   │   │   ├── regions/
│   │   │   │   ├── r_0_0.json   # Region (4×4 chunks)
│   │   │   │   ├── r_0_1.json
│   │   │   │   ├── r_1_0.json
│   │   │   │   └── r_1_1.json
│   │   │   └── image/
│   │   │       ├── preview.png  # CPU-rendered preview
│   │   │       └── layer_0.png  # Optional layers
│   │   └── Page_2/
│   │       ├── manifest.json
│   │       ├── regions/
│   │       │   ├── r_0_0.json
│   │       │   ├── r_0_1.json
│   │       │   ├── r_1_0.json
│   │       │   └── r_1_1.json
│   │       └── image/
│   │           └── preview.png
│   └── Derivatives/
│       ├── manifest.json
│       └── Page_1/
│           ├── manifest.json
│           ├── regions/
│           │   ├── r_0_0.json
│           │   ├── r_0_1.json
│           │   ├── r_1_0.json
│           │   └── r_1_1.json
│           └── image/
│               └── preview.png
└── Physics/
  ├── manifest.json
  └── Mechanics/
    ├── manifest.json
    └── Page_1/
      ├── manifest.json
      ├── regions/
    │   ├── r_0_0.json
    │   ├── r_0_1.json
    │   ├── r_1_0.json
    │   └── r_1_1.json
      └── image/
        └── preview.png


        
Move/Create/Delete a file/folder/project reload entire the tree.
Move a page change nothing except the parent file.
