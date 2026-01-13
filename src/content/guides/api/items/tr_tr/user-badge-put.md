Bu uç nokta, bir kullanıcı rozeti atamasını güncellemenize olanak tanır.

Şu anda yalnızca güncellenebilen özellik `displayedOnComments`'tir; bu özellik rozetin kullanıcının yorumlarında gösterilip gösterilmeyeceğini kontrol eder.

Örnek İstek:

[inline-code-attrs-start title = 'PUT İsteği Örneği'; type = 'bash'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
curl -X PUT "https://fastcomments.com/api/v1/user-badges/badge123?tenantId=demo&API_KEY=DEMO_API_SECRET" \
-H "Content-Type: application/json" \
-d '{
  "displayedOnComments": false
}'
[inline-code-end]

Örnek Yanıt:

[inline-code-attrs-start title = 'Yanıt'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "success"
}
[inline-code-end]

Olası Hata Yanıtları:

[inline-code-attrs-start title = 'Hata: Tenant ID Eksik'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-tenant-id",
  "reason": "Tenant ID (query param: tenantId) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Hata: ID Eksik'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "missing-id",
  "reason": "The User Badge ID (url param: id) is missing."
}
[inline-code-end]

[inline-code-attrs-start title = 'Hata: Bulunamadı'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "status": "failed",
  "code": "not-found",
  "reason": "The user badge badge123 was not found."
}
[inline-code-end]