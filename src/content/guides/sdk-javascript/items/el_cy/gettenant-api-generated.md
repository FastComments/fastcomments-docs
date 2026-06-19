## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp';
const id: string = 'tenant-987654321';
const options: { includeBilling?: boolean } = { includeBilling: true };
const response: GetTenantResponse = await getTenant(tenantId, id);
const billingInfo: BillingInfo | undefined = undefined
[inline-code-end]

---