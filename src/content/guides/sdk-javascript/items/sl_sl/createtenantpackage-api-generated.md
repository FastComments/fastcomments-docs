## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createTenantPackageBody | CreateTenantPackageBody | Yes |  |

## Odgovor

Vrača: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7890";
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: "Growth Plan",
  maxSeats: 2500,
  features: {
    moderation: true,
    analytics: true,
    sso: { enabled: true, provider: "saml" }
  },
  billing: { interval: "monthly", priceCents: 19900 },
  // prikazan neobvezen parameter: notes ni obvezen, vendar je naveden
  notes: "Onboarding bundle with priority support"
};
const result: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
[inline-code-end]