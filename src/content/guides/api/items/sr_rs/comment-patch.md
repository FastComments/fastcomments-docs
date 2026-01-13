[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ова крајња тачка API-ја омогућава ажурирање појединачног коментара.

Напомене:

- Овај API може ажурирати коментарски видгет "у реалном времену" ако је потребно (ово повећава основни `creditsCost` са `1` на `2`).
  - Ово може омогућити миграцију коментара између страница "у реалном времену" (промена `urlId`).
  - Миграције коштају додатна `2` кредита јер се странице предрачунавају и ово је CPU интензивно.
- За разлику од API-ја за креирање, овај API НЕће аутоматски креирати корисничке објекте у нашем систему ако је наведена е-пошта.
- Коментари ажурирани преко овог API-ја и даље могу бити проверавани на спам ако је то по жељи.
- Конфигурације као што је максимална дужина коментара, ако су подешене преко административне странице Customization Rule, примењиваће се овде.
- Да бисте омогућили корисницима да ажурирају текст коментара, можете једноставно назначити `comment` у телу захтева. Ми ћемо генерисати резултујући `commentHTML`.
  - Ако дефинишете и `comment` и `commentHTML` ми нећемо аутоматски генерисати HTML.
  - Ако корисник дода помињања или хештегове у свом новом тексту, то ће и даље бити обрађено као у `POST` API-ју.
- Када ажурирате `commenterEmail` на коментару, најбоље је такође назначити `userId`. У супротном, морате осигурати да корисник са том е-поштом припада вашем тенанту, или ће захтев не успети.  


[inline-code-attrs-start title = 'Минимални пример cURL PATCH захтева за коментар'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH захтева за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Корисник који врши ажурирање. По потреби може се користити да се провери да ли може да уреди коментар.  **/
    contextUserId?: string
	/** Да ли треба да проверимо да ли нови коментар изгледа као спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Да ли коментар треба да се појави "live" корисницима који гледају инстанце коментарског видгета са истим urlId. NOTE: Дуплира трошак кредита са 1 на 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора PATCH захтева за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---