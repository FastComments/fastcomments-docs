## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| fromName | string | Tak |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia sendInvite'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-004';
const id: string = 'comment_9b7f3a2c';
const fromName: string = 'María Hernández';
const response: FlagCommentPublic200Response = await sendInvite(tenantId, id, fromName);
[inline-code-end]

---