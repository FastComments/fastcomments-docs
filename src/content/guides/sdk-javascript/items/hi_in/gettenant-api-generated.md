## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getTenant उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchTenant(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const id: string = "tenant_12345";

    const tenantResponse: GetTenantResponse = await getTenant(tenantId, id);

    // प्रतिक्रिया में वैकल्पिक फ़ील्ड
    const billing: BillingInfo | undefined = tenantResponse.billingInfo;
    const domainConfig: APIDomainConfiguration | undefined = tenantResponse.domainConfiguration;
}
[inline-code-end]