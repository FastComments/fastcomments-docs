---
## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| sendEmail | string | Ne |  |

## Odgovor

Vrne: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer deleteModerator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3b2c9a';
const id: string = 'mod_9c2d1f7b';
const sendEmail: string = 'true';
const response: FlagCommentPublic200Response = await deleteModerator(tenantId, id, sendEmail);
console.log(response);
[inline-code-end]

---