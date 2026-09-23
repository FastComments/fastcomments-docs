## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|------------|-----------|
| tenantId | string | Ναι |  |
| createTenantBody | CreateTenantBody | Ναι |  |

## Απάντηση

Επιστρέφει: [`CreateTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'createTenant Παράδειγμα'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "acme-corp-2024";
  const createTenantBody: CreateTenantBody = {
    name: "Acme Corp",
    domainConfiguration: { domain: "comments.acme.com", sslEnabled: true },
    importedSite: { siteId: "site-123", source: "wordpress" },
    billingInfo: { plan: "enterprise", renewalDate: "2025-01-01" },
    description: "Tenant for Acme Corp's production environment", // προαιρετικό
  };
  const response: CreateTenantResponse = await createTenant(tenantId, createTenantBody);
}
run();
[inline-code-end]