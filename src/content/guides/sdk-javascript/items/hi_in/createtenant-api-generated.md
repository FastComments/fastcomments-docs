## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createTenantBody | CreateTenantBody | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createTenant उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "acme-corp-2024";
  const createTenantBody: CreateTenantBody = {
    name: "Acme Corp",
    domainConfiguration: { domain: "comments.acme.com", sslEnabled: true },
    importedSite: { siteId: "site-123", source: "wordpress" },
    billingInfo: { plan: "enterprise", renewalDate: "2025-01-01" },
    description: "Tenant for Acme Corp's production environment", // वैकल्पिक
  };
  const response: CreateTenantResponse = await createTenant(tenantId, createTenantBody);
}
run();
[inline-code-end]