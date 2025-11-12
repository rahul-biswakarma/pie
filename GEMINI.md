## Project Overview

This project is a video conferencing application called "Pie". It consists of a Next.js web application for the frontend and a Rust-based WebSocket server for the backend. The application uses Supabase for user authentication and real-time communication.

### Frontend (`web` directory)

The frontend is a Next.js application built with React and TypeScript. It uses `bun` as the package manager. Key libraries include:

*   **Next.js:** A React framework for building server-side rendered and static web applications.
*   **Supabase:** For authentication and database services.
*   **Tailwind CSS:** For styling.
*   **Radix UI:** For accessible UI components.
*   **Biome:** For code formatting and linting.

### Backend (`server` directory)

The backend is a WebSocket server built with Rust and the `axum` framework. It handles real-time communication between clients. Key libraries include:

*   **Axum:** A web application framework for Rust.
*   **Tokio:** An asynchronous runtime for Rust.
*   **Supabase-jwt:** For validating Supabase JWTs.
*   **WebRTC:** For handling WebRTC connections.

## Building and Running

### Frontend (`web` directory)

1.  **Install dependencies:**
    ```bash
    bun install
    ```
2.  **Create a `.env.local` file:**
    ```env
    NEXT_PUBLIC_SUPABASE_URL=your_supabase_url
    NEXT_PUBLIC_SUPABASE_ANON_KEY=your_supabase_anon_key
    NEXT_PUBLIC_WS_URL=ws://127.0.0.1:3001/socket
    ```
3.  **Run the development server:**
    ```bash
    bun run dev
    ```

### Backend (`server` directory)

1.  **Create a `.env` file:**
    ```env
    SUPABASE_URL=your_supabase_url
    SUPABASE_ANON_KEY=your_supabase_anon_key
    SUPABASE_JWT_SECRET=your_supabase_jwt_secret
    ```
2.  **Run the development server:**
    ```bash
    cargo run
    ```

## Development Conventions

*   **Code Style:** The frontend uses Biome for code formatting and linting. The backend uses the standard Rust formatting conventions.
*   **Testing:** There are no explicit testing practices documented in the project.
*   **Commits:** Commit messages should be clear and concise.
*   **Branching:** Create a new branch for each new feature or bug fix.
*   **Pull Requests:** Pull requests should be reviewed by at least one other person before being merged.

## Progress Log

**2025-11-10**
*   **WebSocket Integration:**
    *   Integrated `react-use-websocket` library into the frontend for robust, real-time communication.
    *   Created a `SocketProvider` to manage the WebSocket connection lifecycle and provide easy access via a custom hook.
    *   Implemented token-based authentication for the WebSocket connection, fetching the JWT from the Supabase session.
*   **Backend Enhancements:**
    *   Added a heartbeat mechanism (`ping`/`pong`) to the Rust WebSocket server to maintain stable connections.
*   **Code Refactoring:**
    *   Relocated the `AuthContext` to a centralized `contexts` directory in the frontend application.
    *   Committed frontend and backend changes in separate, logical commits.