[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Овај API ендпоинт омогућава брисање коментара.

Напомене:

- Овај API може ажурирати коментарски видгет "live" ако је потребно (ово повећава `creditsCost` са `1` на `2`).
- Овај API ће обрисати све подкоментаре.
- Ако је циљани коментар закључан (`isLocked: true`), захтев се одбија са `code: 'locked'`. Прво откључајте коментар, затим обришите.

[inline-code-attrs-start title = 'Пример cURL захтева за брисање коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура DELETE захтева за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Корисник који врши ажурирање. По потреби се може користити да се провери да ли може да обрише коментар.  **/
    contextUserId?: string
	/** Да ли коментар треба да буде обрисан "live" за кориснике који гледају instance widget-а за коментаре са истим urlId. NAPOMENA: Повећава цену у кредитима са 1 на 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за DELETE захтев за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Укључено при грешци. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Укључено при грешци. **/
    reason?: string
}
[inline-code-end]