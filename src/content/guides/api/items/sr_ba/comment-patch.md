[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Овај API endpoint омогућава ажурирање појединачног коментара.

Напомене:

- Овај API може ажурирати коментарски видџет "live" ако је потребно (ово повећава основни `creditsCost` са `1` на `2`).
  - Ово може учинити миграцију коментара између страница "live" (промјена `urlId`).
  - Миграције коштају додатна `2` кредита јер се странице претрачунавају и ово је интензивно за CPU.
- За разлику од create API, овај API НЕће аутоматски креирати корисничке објекте у нашем систему ако је наведен имејл.
- Коментари ажурирани преко овог API-ја и даље се могу провјерити на спам ако је то жељено.
- Конфигурације као што је максимална дужина коментара, ако су подешене преко администраторске странице Customization Rule, примјењиваће се овдје.
- Да бисте омогућили корисницима да ажурирају текст свог коментара, можете једноставно навести `comment` у тијелу захтјева. Ми ћемо генерисати резултујући `commentHTML`.
  - Ако дефинишете и `comment` и `commentHTML`, нећемо аутоматски генерисати HTML.
  - Ако корисник дода помињања или хештегове у свом новом тексту, они ће и даље бити обрађени као код `POST` API-ја.
- Када ажурирате `commenterEmail` на коментару, најбоље је да такође наведете `userId`. У супротном, морате осигурати да корисник са тим имејлом припада вашем tenant-у, иначе ће захтјев не успјети.  
- Ако је циљни коментар закључан (`isLocked: true`), захтјев се одбија са `code: 'locked'`. Прво откључајте коментар, ажурирајте га, па поново закључајте ако је потребно.


[inline-code-attrs-start title = 'Минимални cURL примјер за PATCH коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH захтјева за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Корисник који врши ажурирање. Ако је потребно, може се користити да се провјери да ли могу уређивати коментар.  **/
    contextUserId?: string
	/** Да ли да провјеримо да ли нови коментар изгледа као спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Да ли би коментар требао бити приказан "live" корисницима који гледају инстанце коментарског видџета са истим urlId. НАПОМЕНА: Повећава трошак кредита са 1 на 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH одговора за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Укључено при неуспјеху. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Укључено при неуспјеху. **/
    reason?: string
}
[inline-code-end]