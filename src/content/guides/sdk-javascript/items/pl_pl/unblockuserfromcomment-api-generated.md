## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Tak |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`UnBlockCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnBlockCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład unBlockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b9c2a';
const id: string = 'comment_4f8e1d';
const unBlockFromCommentParams: UnBlockFromCommentParams = {
  reason: 'User submitted appeal and provided additional context',
  effectiveAt: new Date().toISOString()
};
const userId: string = 'user_92a3f6';
const result: UnBlockCommentPublic200Response = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId);
[inline-code-end]

---