## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| title | string | No |  |
| sso | string | No |  |

## Response

Returns: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Example

[inline-code-attrs-start title = 'createV1PageReact Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "my-tenant-123";
const urlId: string = "article-456";

const pageTitle: string = "Understanding TypeScript Generics";
const ssoToken: string = "user-789-token";

const fullResult: CreateV1PageReact = await createV1PageReact(tenantId, urlId, pageTitle, ssoToken);
const minimalResult: CreateV1PageReact = await createV1PageReact(tenantId, urlId);
[inline-code-end]
