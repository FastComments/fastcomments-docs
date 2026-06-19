## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |
| updateTenantBody | UpdateTenantBody | Sì |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di updateTenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_78f3b2";
const id: string = "tenant-site-01";
const domainConfiguration: APIDomainConfiguration = { primaryDomain: "comments.acme-corp.com", cname: "acme-corp.comments.fastly.net", sslEnabled: true };
const importedSite: ImportedSiteType = { siteId: "blog-42", domain: "blog.acme-corp.com" };
const billingInfo: BillingInfo = { plan: "business", cardLast4: "4242", nextBillingDate: "2026-07-01" };
const updateTenantBody: UpdateTenantBody = { displayName: "Acme Corp", domainConfiguration, importedSites: [importedSite], billingInfo, status: { enabled: true } as APIStatus };
const result: APIEmptyResponse = await updateTenant(tenantId, id, updateTenantBody);
[inline-code-end]

---