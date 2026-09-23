## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Svar

Returnerer: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getTenant Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenant(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const id: string = "tenant_12345";

    const tenantResponse: GetTenantResponse = await getTenant(tenantId, id);

    // Valgfrie felter i svaret
    const billing: BillingInfo | undefined = tenantResponse.billingInfo;
    const domainConfig: APIDomainConfiguration | undefined = tenantResponse.domainConfiguration;
}
[inline-code-end]