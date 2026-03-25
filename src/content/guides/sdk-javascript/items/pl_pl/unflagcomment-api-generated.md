## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| userId | string | Nie |  |
| anonUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`FlagComment200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagComment200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład unFlagComment'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-001';
const commentId: string = 'cmt_9f8e7d6c';
const userId: string = 'user_72b4a1c9';
const anonUserId: string = 'anon_3d2c1b0a';
const response: FlagComment200Response = await unFlagComment(tenantId, commentId, userId, anonUserId);
[inline-code-end]