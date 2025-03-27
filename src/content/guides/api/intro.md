### The FastComments API

FastComments provides an API for interacting with many resources. Build integrations with our platform, or even build your own clients!

In this documentation, you will find all supported resources by the API documented with their request and response types.

For Enterprise customers, all API access is captured in the Audit Log.

### Generated SDKs

FastComments now generates an [API Spec](https://fastcomments.com/js/swagger.json) from our code (this is not yet complete, but includes many APIs).

We also now have SDKs for popular languages:

- [fastcomments-java](https://github.com/FastComments/fastcomments-java)
- [fastcomments-php](https://github.com/FastComments/fastcomments-php)
- [fastcomments-php-sso](https://github.com/FastComments/fastcomments-php-sso)
- Go: Planned
- Python: Planned
- TypeScript: Planned
- Rust: Planned

### Authentication

The API is authenticated by passing your [api key](https://fastcomments.com/auth/my-account/api-secret) as either an `X-API-KEY` header or `API_KEY` query parameter. You will also need your `tenantId`
for making API calls. This can be retrieved from the same page as your api key.

### Security Note

These routes are meant to be called from a **server**. __DO NOT__ call them from a browser. Doing so will expose your API key - this will provide full access to your account
to anyone who can view the source code of a page!

#### Authentication Option One - Headers

- Header: `X-API-KEY`
- Header: `X-TENANT-ID`

#### Authentication Option Two - Query Parameters

- Query Param: `API_KEY`
- Query Param: `tenantId`
