[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ова крајња тачка API-ја омогућава ажурирање појединачног коментара.

Напомене:

- Овај API може, по жељи, ажурирати видгет коментара у реалном времену (ово повећава основни `creditsCost` са `1` на `2`).
  - Ово може омогућити миграцију коментара између страница у реалном времену (промена `urlId`).
  - Миграције коштају додатна `2` кредита јер се странице претходно израчунавају и то је захтјевно за CPU.
- За разлику од API-ја за креирање, овај API НЕће аутоматски креирати корисничке објекте у нашем систему ако је наведена е-пошта.
- Коментари ажурирани преко овог API-ја и даље могу бити провјерени на спам, по жељи.
- Конфигурације као што је максимална дужина коментара, ако су постављене преко странице администрације правила прилагођавања, примјењиваће се овдје.
- Да бисте омогућили корисницима да ажурирају текст свог коментара, можете једноставно навести `comment` у тијелу захтјева. Ми ћемо генерисати резултујући `commentHTML`.
  - Ако дефинишете и `comment` и `commentHTML`, нећемо аутоматски генерисати HTML.
  - Ако корисник дода помињања или хештегове у новом тексту, они ће се и даље обрађивати као код `POST` API-ја.
- Када ажурирате `commenterEmail` на коментару, најбоље је такође навести `userId`. У супротном, морате осигурати да корисник са том е-поштом припада вашем tenant-у, иначе ће захтјев пропасти.  


[inline-code-attrs-start title = 'Минимални cURL примјер за PATCH коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за PATCH коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Корисник који врши ажурирање. По потреби се може користити за провјеру да ли могу уређивати коментар.  **/
    contextUserId?: string
	/** Да ли да провјеримо да ли нови коментар изгледа као спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Да ли коментар треба да се појави у реалном времену корисницима који гледају инстанце видгета коментара са истим urlId-ом. НАПОМЕНА: Повећава трошак кредита са 1 на 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за PATCH коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Укључено у случају неуспјеха. **/
    reason?: string
}
[inline-code-end]