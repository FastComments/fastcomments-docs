## Parameters

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |
| sendEmail | string | Nie |  |

## Odpowiedź

Zwraca: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład deleteModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_9f3b2d4a6c';
const moderatorId: string = 'moderator_84a1b9c2';
const sendEmail: string = 'true';
const result: FlagCommentPublic200Response = await deleteModerator(tenantId, moderatorId, sendEmail);
[inline-code-end]

---