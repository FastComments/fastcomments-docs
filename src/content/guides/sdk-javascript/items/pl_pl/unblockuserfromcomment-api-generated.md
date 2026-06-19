## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| unBlockFromCommentParams | UnBlockFromCommentParams | Tak |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UnblockSuccess.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład unBlockUserFromComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8b4a2f9c';
const id: string = 'cmt_5f3b2a9e';
const unBlockFromCommentParams: UnBlockFromCommentParams = { reason: 'Appeal accepted', effectiveAt: '2026-06-19T12:00:00Z' };
const userId: string = 'user_42f7';
const result: UnblockSuccess = await unBlockUserFromComment(tenantId, id, unBlockFromCommentParams, userId);
[inline-code-end]

---