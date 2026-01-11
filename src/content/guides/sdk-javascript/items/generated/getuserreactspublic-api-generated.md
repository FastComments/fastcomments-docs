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
const tenantId: string = 'tenant_8a9f3c9b';
const postIds: string[] = ['post_4f2a1b6c', 'post_77b8c3d2'];
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySWQiOiI1Njc4OSIsIm5hbWUiOiJKb2huIERvZSJ9.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c';
const response: GetUserReactsPublic200Response = await getUserReactsPublic(tenantId, postIds, sso);
console.log(response);
[inline-code-end]
