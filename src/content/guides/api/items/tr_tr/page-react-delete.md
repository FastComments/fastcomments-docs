[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Mevcut kullanıcının bir sayfadaki tepkilerinden birini kaldırır. Kullanıcı bu tepkiyi eklememişse, istek `no-react` kodu ile başarılı olur ve sayım değişmez.

[inline-code-attrs-start title = 'Sayfa Tepki Silme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Tepki Silme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** Tepkinin kimliği. **/
    id: string
    /** SSO nesnenizin URI kodlu JSON'u. Anonim kullanıcılar için atlayın. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Sayfa Tepki Silme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** Kullanıcı bu tepkiyi eklememişse 'no-react'. Aksi takdirde hata durumunda dahil edilir. **/
    code?: 'no-react' | string
    /** Hata durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]

---