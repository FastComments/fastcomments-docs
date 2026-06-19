## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwort

Gibt zurück: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getTenant Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = 'tenant-987654321';
const options: { includeBilling?: boolean } = { includeBilling: true };
const response: GetTenantResponse = await getTenant(tenantId, id);
const billingInfo: BillingInfo | undefined = undefined
[inline-code-end]

---