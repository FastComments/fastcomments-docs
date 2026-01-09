[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ова крајња тачка API‑ја омогућава брисање коментара.

Напомене:

- Ова крајња тачка API‑ја може, по жељи, ажурирати виджет коментара "уживо" (ово повећава `creditsCost` са `1` на `2`).
- Овај API ће обрисати све подкоментаре.

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
	/** Корисник који врши ажурирање. По потреби се може користити за проверу да ли може обрисати коментар.  **/
    contextUserId?: string
	/** Да ли коментар треба бити обрисан "уживо" за кориснике који гледају инстанце видгета коментара са истим urlId. НАПОМЕНА: Удвостручује трошак у кредитима са 1 на 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за брисање коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---