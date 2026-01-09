[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт омогућава брисање коментара.

Напомене:

- Овај API може ажурирати видгет коментара "у реалном времену" ако је то жељено (ово повећава `creditsCost` са `1` на `2`).
- Овај API ће избрисати све подкоментаре.

[inline-code-attrs-start title = 'Пример cURL захтева за брисање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура захтева за брисање коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Корисник који врши ажурирање. Ако је потребно, може се користити да се провери да ли тај корисник може обрисати коментар.  **/
    contextUserId?: string
	/** Да ли би коментар требало бити обрисан "у реалном времену" за кориснике који прегледају инстанце видгета коментара са истим urlId. НАПОМЕНА: Дуплира трошак кредита са 1 на 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за брисање коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Укључено у случају неуспјеха. **/
    reason?: string
}
[inline-code-end]

---