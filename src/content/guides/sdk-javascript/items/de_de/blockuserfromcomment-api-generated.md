## Parameter

| Name | Type | Erforderlich | Beschreibung |
|------|------|--------------|--------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| blockFromCommentParams | BlockFromCommentParams | Ja |  |
| userId | string | Nein |  |
| anonUserId | string | Nein |  |

## Antwort

Gibt zurück: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für blockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const id: string = "comment_67890";
const blockFromCommentParams: BlockFromCommentParams = {
  reason: "Repeated abusive language",
  blockDurationHours: 168,
  blockReplies: true,
  notifyAuthor: true
};
const userId: string = "user_abc123";
const anonUserId: string = "anon_xyz789";
const result: BlockFromCommentPublic200Response = await blockUserFromComment(tenantId, id, blockFromCommentParams, userId, anonUserId);
[inline-code-end]

---