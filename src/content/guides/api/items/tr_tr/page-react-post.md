[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Mevcut kullanıcı olarak bir sayfaya bir reaksiyon ekler. Bir kullanıcı her reaksiyon kimliğini bir kez ekleyebilir: tekrar eklemek `already-reacted` kodu ile başarılı olur ve sayıyı değiştirmez. Bir kullanıcı aynı sayfaya birkaç farklı reaksiyon ekleyebilir.

Reaksiyon kimlikleri sizin tarafınızdan seçilir ve en fazla 36 karakter olabilir. Sayfa henüz yoksa oluşturulur. Sayfanın başlığını ayarlamak veya güncellemek için `title` parametresini gönderin.

[inline-code-attrs-start title = 'Sayfa Reaksiyon cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Reaksiyon İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** Reaksiyon kimliği, en fazla 36 karakter. **/
    id: string
    /** Sayfanın başlığını ayarlar. **/
    title?: string
    /** SSO nesnenizin URI kodlu JSON'u. Anonim kullanıcılar için atlayın. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Reaksiyon Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** Kullanıcı zaten bu reaksiyonu eklediğinde 'already-reacted' döner. Kimlik 36 karakteri aştığında (HTTP 422) 'react-id-too-long' döner. Aksi takdirde hata durumunda dahil edilir. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Hata durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]