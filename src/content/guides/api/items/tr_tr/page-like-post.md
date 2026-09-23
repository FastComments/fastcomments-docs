[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Mevcut kullanıcı olarak bir sayfayı beğenir. Her kullanıcı bir sayfayı bir kez beğenebilir: tekrar beğenmek `already-liked` kodu ile başarılı olur ve sayıyı değiştirmez.

Sayfa henüz yoksa oluşturulur. Sayfanın başlığını ayarlamak veya güncellemek için `title` parametresini gönderin.

[inline-code-attrs-start title = 'Sayfa Beğenme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Beğenme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Sayfanın başlığını ayarlar. **/
    title?: string
    /** SSO nesnenizin URI kodlu JSON'u. Anonim kullanıcılar için atlayın. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Beğenme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' kodu, kullanıcı sayfayı zaten beğenmiş olduğunda döner. Aksi takdirde hata durumunda dahil edilir. **/
    code?: 'already-liked' | string
    /** Hata durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]