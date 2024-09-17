# Simple File Server

A basic file server written in Rust that serves files from a specified directory over TCP. This server handles HTTP requests, serves static files, and responds with appropriate HTTP headers.

## Features

- Serve static files and directories.
- Support for various MIME types including HTML, CSS, JavaScript, images, and videos.
- Basic error handling with HTTP 404 for not found errors.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/fullendmaestro/simple-file-server.git

2. Navigate into the project directory:

   ```sh
   cd simple-file-server

3. Install the required dependencies:

   ```sh
   cargo build


### Running the Server

1. Run the server with the following command:

   ```sh
   cargo run



2. By default, the server will listen on port 5500. You can access it via:

   ```sh
   http://localhost:5500
   

## Usage

- **Serving Files**: 
  - Place your files in the project's root directory or a subdirectory. 
  - The server will serve these files based on the request path. 
  - For example, if you have a file named `example.html` in the root directory, you can access it via:
    ```bash
    http://localhost:5500/example.html
    ```

- **Directory Listings**:
  - If a directory is requested, the server will list the contents of the directory.
  - For instance, if you request a directory like `files/`, the server will display a list of files and subdirectories within `files/`.
  - (Note: Directory listing functionality may need to be implemented if it is not yet available in the current version of the server.)


## Error Handling

- **404 Not Found**: 
  - If the requested file or directory does not exist, the server will respond with a 404 error.



## Dependencies

- **infer**: 
  - For MIME type detection based on file extension.
- **infer**: 
  - For MIME type detection based on file extension.