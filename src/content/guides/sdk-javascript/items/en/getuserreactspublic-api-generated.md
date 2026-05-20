## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| postIds | Array<string> | No |  |
| sso | string | No |  |

## Response

Returns: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'getUserReactsPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-01a9f3b6";
const postIds: Array<string> = ["post-8f3d2c1e", "post-4b2e9a7d"];
const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature";
const responseWithPosts: GetUserReactsPublic200Response = await getUserReactsPublic(tenantId, postIds, sso);
const responseMinimal: GetUserReactsPublic200Response = await getUserReactsPublic(tenantId);
[inline-code-end]
