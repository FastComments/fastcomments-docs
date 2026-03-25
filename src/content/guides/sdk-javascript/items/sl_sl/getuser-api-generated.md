## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrne: [`GetUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUser200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7b3f42';
const id: string = 'user_9c4d2a';
const userResponse: GetUser200Response = await getUser(tenantId, id);
console.log(userResponse);
[inline-code-end]

---