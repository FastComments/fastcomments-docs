[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт омогућава брисање коментара.

Напомене:

- Овај API може ажурирати видгет коментара "live" ако је потребно (ово увећава `creditsCost` са `1` на `2`).
- Овај API ће избрисати све подкоментаре.

[inline-code-attrs-start title = 'cURL пример за брисање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** Корисник који врши ажурирање. По потреби се може користити да се провери да ли он/она може обрисати коментар.  **/
    contextUserId?: string
	/** Да ли коментар треба да буде обрисан "live" за кориснике који гледају инстанце видгета за коментаре са истим urlId. НАПОМЕНА: Удвостручује трошак кредита са 1 на 2. **/
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