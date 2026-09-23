## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse.ts)

## Пример

[inline-code-attrs-start title = 'getTenant Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenant(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const id: string = "tenant_12345";

    const tenantResponse: GetTenantResponse = await getTenant(tenantId, id);

    // Опционална поља у одговору
    const billing: BillingInfo | undefined = tenantResponse.billingInfo;
    const domainConfig: APIDomainConfiguration | undefined = tenantResponse.domainConfiguration;
}
[inline-code-end]

---