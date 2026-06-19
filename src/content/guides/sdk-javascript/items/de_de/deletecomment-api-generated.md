## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| contextUserId | string | Nein |  |
| isLive | boolean | Nein |  |

## Antwort

Gibt zurück: [`DeleteCommentResult`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteCommentResult.ts)

## Beispiel

[inline-code-attrs-start title = 'deleteComment Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-834';
const id: string = 'cmt_9f3b2d7a';
const contextUserId: string = 'user_4b2f6c88-1a2b-4c3d-9e5f-123456789abc';
const isLive: boolean = true;
const result: DeleteCommentResult = await deleteComment(tenantId, id, contextUserId, isLive);
[inline-code-end]