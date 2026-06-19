## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = 'tenant-987654321';
const options: { includeBilling?: boolean } = { includeBilling: true };
const response: GetTenantResponse = await getTenant(tenantId, id);
const billingInfo: BillingInfo | undefined = undefined
[inline-code-end]