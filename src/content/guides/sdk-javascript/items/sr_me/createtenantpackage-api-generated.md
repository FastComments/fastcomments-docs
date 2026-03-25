## Parametri

| Name | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| createTenantPackageBody | CreateTenantPackageBody | Da |  |

## Odgovor

Vraća: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer createTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: 'Standard Moderation',
  description: 'Suitable for small-to-medium sites: basic moderation, spam rules, and analytics',
  maxCommentsPerMinute: 50,
  allowAnonymousComments: false, // opcioni parametar je prosleđen
  // opcionalna polja izostavljena: npr., napredna pravila moderacije, prilagođeni CSS
  customConfigParameters: {
    enableProfanityFilter: true,
    imageContentProfanityLevel: 'medium' // ilustrativna vrijednost; koristi oblik CustomConfigParameters
  }
};
const response: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
console.log(response);
[inline-code-end]