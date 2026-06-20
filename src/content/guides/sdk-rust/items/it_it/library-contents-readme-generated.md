---
The FastComments Rust SDK consists of several modules:

- **Client Module** - API client for FastComments REST APIs
  - Complete type definitions for all API models
  - Three API clients covering all FastComments methods:
    - `default_api` (**DefaultApi**) - API-key-authenticated methods for server-side use
    - `public_api` (**PublicApi**) - public, no-API-key methods that are safe to call from browsers and mobile apps
    - `moderation_api` (**ModerationApi**) - methods backing the moderator dashboard, including comment moderation (list, count, search, logs, export), moderation actions (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread), bans (ban from a comment, undo, pre-ban summaries, ban status/preferences, banned-user counts), and badges & trust (award/remove badges, manual badges, get/set trust factor, user internal profile). Every Moderation method accepts an `sso` parameter so the call can be made on behalf of an SSO-authenticated moderator.
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
---