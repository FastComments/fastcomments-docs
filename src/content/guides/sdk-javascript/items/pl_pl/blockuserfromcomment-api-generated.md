## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| blockFromCommentParams | BlockFromCommentParams | Tak |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/BlockFromCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład blockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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