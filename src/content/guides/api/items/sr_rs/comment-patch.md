[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ова крајња тачка API-ја омогућава ажурирање појединачног коментара.

Напомене:

- Овај API може ажурирати коментарски виџет уживо ако се жели (ово повећава основни `creditsCost` са `1` на `2`).
  - Ово омогућава да миграције коментара између страница буду уживо (променом `urlId`).
  - Миграције коштају додатна `2` кредита јер се странице предрачунавају и то је CPU интензивно.
- За разлику од API-ја за креирање, овај API НЕће аутоматски креирати корисничке објекте у нашем систему ако је наведена е-пошта.
- Коментари ажурирани преко овог API-ја и даље се могу проверавати на спам ако желите.
- Поставке као што је максимална дужина коментара, ако су подешене преко админ странице Customization Rule, важе овде.
- Да бисте омогућили корисницима да ажурирају текст свог коментара, можете у телу захтева једноставно назначити `comment`. Ми ћемо генерисати резултујући `commentHTML`.
  - Ако наведете и `comment` и `commentHTML`, ми нећемо аутоматски генерисати HTML.
  - Ако корисник дода помене или хештегове у свој нови текст, то ће и даље бити обрађено као и у `POST` API-ју.
- Када ажурирате `commenterEmail` на коментару, најбоље је такође назначити `userId`. У супротном, морате да се уверите да корисник са овом е-поштом припада вашем tenant-у, иначе ће захтев пропасти.  
- Ако је циљани коментар закључан (`isLocked: true`), захтев се одбија са `code: 'locked'`. Прво откључајте коментар, ажурирајте га, а затим га поново закључајте ако желите.


[inline-code-attrs-start title = 'Минимални пример cURL PATCH за коментар'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** Корисник који извршава ажурирање. Може се користити за проверу да ли имају право да уређују коментар.  **/
    contextUserId?: string
	/** Да ли треба проверити да ли нови коментар личи на спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Да ли коментар треба да се појави "уживо" корисницима који гледају инстанце коментарског виџета са истим urlId. НАПОМЕНА: Дуплира трошак кредита са 1 на 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура PATCH одговора за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Укључено при неуспеху. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Укључено при неуспеху. **/
    reason?: string
}
[inline-code-end]

---