## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nee |  |

## Response

Retourneert: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenantUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_7f3b2a9c';
  const skip: number = 20; // optionele parameter ter demonstratie
  const result: GetTenantUsersResponse = await getTenantUsers(tenantId, skip);
  console.log(result);
})();
[inline-code-end]

---