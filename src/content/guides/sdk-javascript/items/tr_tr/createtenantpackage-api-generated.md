## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createTenantPackageBody | CreateTenantPackageBody | Evet |  |

## Yanıt

Döndürür: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateTenantPackage200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createTenantPackage Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const createTenantPackageBody: CreateTenantPackageBody = {
  packageName: 'Standard Moderation',
  description: 'Suitable for small-to-medium sites: basic moderation, spam rules, and analytics',
  maxCommentsPerMinute: 50,
  allowAnonymousComments: false, // isteğe bağlı parametre sağlandı
  // isteğe bağlı alanlar dahil edilmedi: örn., gelişmiş moderasyon kuralları, özel CSS
  customConfigParameters: {
    enableProfanityFilter: true,
    imageContentProfanityLevel: 'medium' // örnek değer; CustomConfigParameters yapısını kullanır
  }
};
const response: CreateTenantPackage200Response = await createTenantPackage(tenantId, createTenantPackageBody);
console.log(response);
[inline-code-end]

---