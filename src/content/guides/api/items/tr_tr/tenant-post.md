[api-resource-header-start name = 'Tenant'; route = 'POST /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Bu rota, tek bir `Tenant` ekleme olanağı sağlar.

Bir `Tenant` oluşturmak aşağıdaki kısıtlamalara sahiptir:

- Bir `name` zorunludur.
- `domainConfiguration` zorunludur.
- Bir `Tenant` oluşturulurken aşağıdaki değerler verilemez:
  - `hasFlexPricing` 
  - `lastBillingIssueReminderDate` 
  - `flexLastBilledAmount`
- `signUpDate` gelecekte bir tarih olamaz.
- `name` değeri `200 karakterden` uzun olamaz.
- `email` değeri `300 karakterden` uzun olamaz.
- `email`, FastComments.com üzerindeki tüm tenant'lar arasında benzersiz olmalıdır.
- Üst tenant'ta geçerli bir `TenantPackage` tanımlı değilse tenant oluşturamazsınız.
  - Tenant'ınız FastComments.com üzerinden oluşturulduysa bu bir sorun olmamalıdır.
- Paketinizde `maxWhiteLabeledTenants` altında tanımlanan sayıdan fazla tenant oluşturamazsınız.
- White labeling etkinleştirilmiş `parent tenant`'ınızın kimliği olan `tenantId` sorgu parametresini belirtmelisiniz.

Yalnızca birkaç parametre ile bir `Tenant` oluşturabiliriz:

[inline-code-attrs-start title = 'Tenant Oluşturma cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "domainConfiguration": [ { "domain": "somedomain.com" } ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Oluşturma İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant Oluşturma Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPostResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'not-found' | 'unexpected-param' | 'sign-up-date-in-future' | 'payment-frequency-invalid' | 'cannot-change-payment-frequency' | 'name-invalid' | 'email-invalid' | 'email-taken' | 'no-package' | 'invalid-package' | 'unauthorized' | 'tenant-limit-reached' | 'cannot-move-tenant' | 'cannot-change-package' | 'invalid-billing-info'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    tenant?: Tenant; // Başarı durumunda oluşturulan tenant'ın tamamını döndürürüz.
}
[inline-code-end]