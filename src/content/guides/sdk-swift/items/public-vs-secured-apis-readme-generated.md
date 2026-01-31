The FastComments SDK provides two types of API endpoints:

### PublicAPI - Client-Safe Endpoints

The `PublicAPI` contains endpoints that are safe to call from client-side code (iOS/macOS apps). These endpoints:
- Do not require an API key
- Can use SSO tokens for authentication
- Are rate-limited per user/device
- Are suitable for end-user facing applications

**Example use case**: Fetching and creating comments in your iOS app

### DefaultAPI - Server-Side Endpoints

The `DefaultAPI` contains authenticated endpoints that require an API key. These endpoints:
- Require your FastComments API key
- Should ONLY be called from server-side code
- Provide full access to your FastComments data
- Are rate-limited per tenant

**Example use case**: Administrative operations, bulk data export, moderation tools

**IMPORTANT**: Never expose your API key in client-side code. API keys should only be used server-side.