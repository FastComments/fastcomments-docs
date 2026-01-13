## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| meta | string | Όχι |  |
| skip | number | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_9f2d1b7c';
  const meta: string = 'include=domains,billing,customConfig';
  const skip: number = 20;
  const response: GetTenants200Response = await getTenants(tenantId, meta, skip);
  console.log(response);
})();
[inline-code-end]

---