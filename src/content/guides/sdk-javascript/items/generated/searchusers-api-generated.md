## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| usernameStartsWith | string | Yes |  |
| mentionGroupIds | Array<string> | No |  |
| sso | string | No |  |

## Response

Returns: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsers200Response.ts)

## Example

[inline-code-attrs-start title = 'searchUsers Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_corp";
const urlId: string = "news/2025/11/22/site-launch";
const usernameStartsWith: string = "jo";
const mentionGroupIds: Array<string> = ["senior-editors", "community-moderators"];
const sso: string = "sso_session_9f8e7d6c5b4a3";
const result: SearchUsers200Response = await searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso);
[inline-code-end]
