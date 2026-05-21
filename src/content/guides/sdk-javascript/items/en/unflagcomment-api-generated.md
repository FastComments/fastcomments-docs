## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| userId | string | No |  |
| anonUserId | string | No |  |

## Response

Returns: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## Example

[inline-code-attrs-start title = 'unFlagComment Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8247';
const id: string = 'cmt_48291';
const userId: string = 'user_jdoe42';
const anonUserId: string = 'anon_5f9a';

const resultWithUser: FlagComment200Response = await unFlagComment(tenantId, id, userId);
const resultWithAnon: FlagComment200Response = await unFlagComment(tenantId, id, undefined, anonUserId);
[inline-code-end]
