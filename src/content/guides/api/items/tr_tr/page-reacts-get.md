[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Sayfa üzerindeki her reaksiyonun sayısını ve mevcut kullanıcının eklediği reaksiyonları döndürür.

[inline-code-attrs-start title = 'Sayfa Reaksiyonları cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Reaksiyonları İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** SSO nesnenizin URI kodlu JSON'u. Anonim kullanıcılar için atlayın. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Reaksiyonları Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Başarısızlıkta dahil edilir. **/
    code?: string
    /** Başarısızlıkta dahil edilir. **/
    reason?: string
    /** Her reaksiyon kimliği için sayı, örneğin {"heart": 12, "laugh": 3}. Sayfa hiç reaksiyon içermediğinde ayarlanmaz. **/
    counts?: Record<string, number>
    /** İsteği yapan kullanıcının eklediği reaksiyon kimlikleri. Hiç eklemediklerinde ayarlanmaz. **/
    reactedIds?: string[]
}
[inline-code-end]

---