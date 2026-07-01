The FastComments Rust SDK consists of several modules:

- **Client Module** - API client for FastComments REST APIs
  - Complete type definitions for all API models
  - Three API clients covering all FastComments methods:
    - `default_api` (**DefaultApi**) - API-key-authenticated methods for server-side use
    - `public_api` (**PublicApi**) - public, no-API-key methods that are safe to call from browsers and mobile apps
    - `moderation_api` (**ModerationApi**) - an extensive suite of live and fast moderation APIs. Every Moderation method accepts an `sso` parameter and can authenticate via SSO or a FastComments.com session cookie.
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