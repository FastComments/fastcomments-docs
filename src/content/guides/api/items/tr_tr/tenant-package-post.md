[api-resource-header-start name = 'TenantPackage'; route = 'POST /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir `TenantPackage` ekleme yeteneği sağlar.

Bir `TenantPackage` oluşturmanın aşağıdaki kısıtlamaları vardır:

- Aşağıdaki parametreler gereklidir:
    - `name`
    - `tenantId`
    - `monthlyCostUSD` - Boş (null) olabilir.
    - `yearlyCostUSD` - Boş (null) olabilir.
    - `maxMonthlyPageLoads`
    - `maxMonthlyAPICredits`
    - `maxMonthlyComments`
    - `maxConcurrentUsers`
    - `maxTenantUsers`
    - `maxSSOUsers`
    - `maxModerators`
    - `maxDomains`
    - `hasDebranding`
    - `forWhoText`
    - `featureTaglines`
    - `hasFlexPricing` - Eğer true ise, tüm `flex*` parametreleri gereklidir.
- `name` `50 characters`'den daha uzun olamaz.
- Her `forWhoText` öğesi `200 characters`'den daha uzun olamaz.
- Her `featureTaglines` öğesi `100 characters`'den daha uzun olamaz.
- `TenantPackage`, üst kiracıdan daha "küçük" olmalıdır. Örneğin, tüm `max*` parametrelerinin üst kiracıdan daha düşük değerlere sahip olması gerekir. 
- Bir beyaz etiketli kiracı en fazla **beş pakete** sahip olabilir.
- Sadece beyaz etiketleme erişimine sahip kiracılar bir `TenantPackage` oluşturabilir.
- Kendi kiracınıza paket ekleyemezsiniz. :)

Bir `TenantPackage` şu şekilde oluşturulabilir:

[inline-code-attrs-start title = 'TenantPackage E-posta ile Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
  "name": "Default Package",
  "tenantId": "some-child-tenant-id",
  "monthlyCostUSD": null,
  "yearlyCostUSD": null,
  "maxMonthlyPageLoads": 50000,
  "maxMonthlyAPICredits": 50000,
  "maxMonthlyComments": 50000,
  "maxConcurrentUsers": 50000,
  "maxTenantUsers": 10,
  "maxSSOUsers": 50000,
  "maxModerators": 100,
  "maxDomains": 3,
  "hasWhiteLabeling": false,
  "hasDebranding": true,
  "forWhoText": "For Everyone",
  "featureTaglines": [
    "Some Tag",
    "Some Other Tag"
  ],
  "hasFlexPricing": true,
  "flexPageLoadCostCents": 100,
  "flexPageLoadUnit": 100000,
  "flexCommentCostCents": 100,
  "flexCommentUnit": 100000,
  "flexSSOUserCostCents": 100,
  "flexSSOUserUnit": 1000,
  "flexAPICreditCostCents": 100,
  "flexAPICreditUnit": 50000,
  "flexModeratorCostCents": 500,
  "flexModeratorUnit": 1,
  "flexAdminCostCents": 1000,
  "flexAdminUnit": 1,
  "flexDomainCostCents": 1000,
  "flexDomainUnit": 1,
  "flexMinimumCostCents": 99
}'
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Oluşturma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantPackage Oluşturma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagePostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' 'white-labeling-not-allowed' | 'name-too-long' | 'for-who-text-too-long' | 'feature-tag-lines-too-long' | 'no-package' | 'invalid-package' | 'unauthorized' | 'child-tenant-too-large' | 'flex-param-missing' | 'unexpected-flex-param' | 'package-limit-reached' | 'flex-param-missing' | 'unexpected-flex-param'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    tenantPackage?: TenantPackage; // Başarı durumunda oluşturulan tenant paketinin tamamını döneriz.
}
[inline-code-end]

---