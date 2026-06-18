## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| postId | string | כן |  |
| broadcastId | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublic200Response.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteFeedPostPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const postId: string = 'post_8f3d2a7c';
const broadcastId: string = 'broadcast_2026-06-15_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';
const response: DeleteFeedPostPublic200Response = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
[inline-code-end]

---