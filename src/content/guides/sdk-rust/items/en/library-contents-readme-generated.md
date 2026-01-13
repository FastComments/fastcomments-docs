The FastComments Rust SDK consists of several modules:

- **Client Module** - Auto-generated API client for FastComments REST APIs
  - Complete type definitions for all API models
  - Both authenticated (`DefaultApi`) and public (`PublicApi`) endpoints
  - Full async/await support with tokio
  - See [client/README.md](https://github.com/FastComments/fastcomments-rust/blob/main/client/README.md) for detailed API documentation

- **SSO Module** - Server-side Single Sign-On utilities
  - Secure token generation for user authentication
  - Support for both simple and secure SSO modes
  - HMAC-SHA256 based token signing

- **Core Types** - Shared type definitions and utilities
  - Comment models and metadata structures
  - User and tenant configurations
  - Helper functions for common operations