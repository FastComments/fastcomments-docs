[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ова крајња тачка API-ја омогућава креирање коментара.

Уобичајени случајеви употребе су прилагођени интерфејси, интеграције или увоз.

Напомене:

- Овај API може ажурирати видгет коментара "уживо" ако је потребно (ово повећава `creditsCost` са `1` на `2`).
- Овај API ће аутоматски креирати објекте корисника у нашем систему ако је наведена е-пошта.
- Покушај да се сачувају два коментара са различитим е-поштама, али истим корисничким именом, резултираће грешком за други коментар.
- Ако наводите `parentId`, и дечји коментар има `notificationSentForParent` као false, **послаћемо обавештења за родитељски коментар**. Ово се ради сваки сат (групишемо обавештења заједно да бисмо смањили број послатих имејлова).
- Ако желите да пошаљете добродошлине имејлове приликом креирања корисника, или имејлове за верификацију коментара, подесите `sendEmails` на `true` у параметрима упита.
- Коментари креирани преко овог API-ја ће се појавити на страницама Analytics и Moderation админ апликације.
- "bad words" су и даље маскиране у именима коментатора и тексту коментара ако је то подешавање укључено.
- Коментари креирани преко овог API-ја и даље се могу проверавати на спам ако је потребно.
- Конфигурације као што је максимална дужина коментара, ако су подешене преко странице Customization Rule у админ панелу, примењиваће се овде.

Минимални подаци потребни за слање који ће се приказати у видгету коментара су следећи:

[inline-code-attrs-start title = 'Минимални cURL пример за POST коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'cURL пример за POST коментара'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура захтева за POST коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPostQueryParams {
    tenantId: string
    API_KEY: string
    doSpamCheck?: 'true' | 'false'
	/** Да ли коментар треба да се приказује "уживо" корисницима који гледају инстанце видгета коментара са истим urlId-ом. НАПОМЕНА: Удваја трошак кредита са 1 на 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за POST коментара'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Укључено у случају неуспеха. **/
    reason?: string
    /** Креирани коментар. **/
    comment?: Comment
    /** Повезани корисник, који је можда већ постојао, а можда и није. **/
    user?: User
}
[inline-code-end]

---