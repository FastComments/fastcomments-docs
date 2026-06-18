## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|-------------|-------------|
| tenantId | string | Ja |  |
| postId | string | Ja |  |
| broadcastId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteFeedPostPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteFeedPostPublic Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-42';
const postId: string = 'post_8f3d2a7c';
const broadcastId: string = 'broadcast_2026-06-15_01';
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.ssoPayload.signature';
const response: DeleteFeedPostPublic200Response = await deleteFeedPostPublic(tenantId, postId, broadcastId, sso);
[inline-code-end]

---