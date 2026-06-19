---
## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getTenantUsers Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_7f3b2a9c';
  const skip: number = 20; // demonstracija opcionalnog parametra
  const result: GetTenantUsersResponse = await getTenantUsers(tenantId, skip);
  console.log(result);
})();
[inline-code-end]

---