## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |
| fromName | string | Da |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Primer

[inline-code-attrs-start title = 'sendInvite Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant-987654321";
const id: string = "user-123456789";
const fromName: string = "Alice Johnson";

const result: APIEmptyResponse = await sendInvite(tenantId, id, fromName);
console.log(result);
[inline-code-end]

---