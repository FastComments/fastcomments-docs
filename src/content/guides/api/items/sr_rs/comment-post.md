[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Овај API крајњи пут омогућава креирање коментара.

Уобичајене примене су прилагођени кориснички интерфејси, интеграције или увози.

Напомене:

- Овај API може ажурирати видгет за коментаре "уживо" ако је потребно (ово повећава `creditsCost` са `1` на `2`).
- Овај API ће аутоматски креирати објекте корисника у нашем систему ако је е-пошта наведена.
- Покушај да се сачувају два коментара са различитим е-поштама, али истим корисничким именом, довешће до грешке за други коментар.
- Ако наведете `parentId`, и ако дечји коментар има `notificationSentForParent` као false, **послаћемо обавештења за родитељски коментар**. Ово се ради сваки сат (групишемо обавештења како бисмо смањили број послатих е-порука).
- Ако желите да пошаљете поздравне е-поруке при креирању корисника, или е-поруке за верификацију коментара, поставите `sendEmails` на `true` у параметрима упита.
- Коментари креирани преко овог API-ја ће се појавити на страницама Аналитике и Модерације у админ апликацији.
- „лоше речи“ се и даље замагљују у именима коментатора и тексту коментара ако је подешавање укључено.
- Коментари креирани путем овог API-ја и даље се могу проверити на спам ако је то потребно.
- Конфигурација као што је максимална дужина коментара, ако је подешена преко странице правила прилагођавања у админ интерфејсу, примењиваће се и овде.

Минимални подаци потребни за слање који ће се приказати у видгету за коментаре су следећи:

[inline-code-attrs-start title = 'Минималан пример cURL POST захтева за коментар'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"date": 1622644382148,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true
}'
[inline-code-end]

Реалнији захтев може изгледати овако:

[inline-code-attrs-start title = 'Реалнији пример cURL POST захтева за коментар'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true&doSpamCheck=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"approved": true,
	"avatarSrc": "https://static.fastcomments.com/1605337537848-DSC_0841.JPG",
	"comment": "some-comment",
	"commenterName": "some-commenter-name",
	"commenterEmail": "fordperfect@spaceship.com",
	"date": 1622644382148,
	"isSpam": false,
	"locale": "en_us",
	"notificationSentForParent": true,
	"notificationSentForParentTenant": true,
	"reviewed": true,
	"urlId": "some-place",
	"url": "https://exmaple.com/some-page",
	"verified": true,
	"votes": 1,
	"votesUp": 2,
	"votesDown": 1,
	"ip": "123.456.789.000"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура POST захтева за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Да ли коментар треба да се приказује "уживо" корисницима који гледају инстанце видгета за коментаре са истим urlId-ом. НАПОМЕНА: Удваја трошак кредита са 1 на 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора POST захтева за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Креирани коментар. **/
    comment?: Comment
    /** Повезани корисник, који можда већ постоји или можда није постојао. **/
    user?: User
}
[inline-code-end]