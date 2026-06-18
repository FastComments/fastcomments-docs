## Παράμετροι

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| id | string | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_ab12c3';
const id: string = 'user_9f8e7d';
const response: GetTenantUser200Response = await getTenantUser(tenantId, id);
console.log(response);
[inline-code-end]

---