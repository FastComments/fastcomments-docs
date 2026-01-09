[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments'; creditsCost = 1; api-resource-header-end]

Ова API крајња тачка омогућава креирање коментара.

Уобичајени случајеви употребе су прилагођени кориснички интерфејси, интеграције или увози.

Напомене:

- Овај API може ажурирати видгет коментара „у живо“ ако је потребно (ова опција повећава `creditsCost` са `1` на `2`).
- Овај API ће аутоматски креирати објекте корисника у нашем систему ако је е-пошта обезбеђена.
- Покушај да се сачувају два коментара са различитим е-поштама, али истим корисничким именом, резултираће грешком за други коментар. 
- Ако одредите `parentId`, и ако подкоментар има `notificationSentForParent` постављено на false, **послаћемо обавештења за родитељски коментар**. То се ради на сваких сат времена (групишемо обавештења заједно да бисмо смањили број послатих имејлова).
- Ако желите слање добродошлих имејлова при креирању корисника, или имејлова за верификацију коментара, поставите `sendEmails` на `true` у параметрима упита.
- Коментари креирани овим API-јем ће се појавити на страницама Аналитике и Модерације у административној апликацији.
- „лоше речи“ су и даље маскиране у именима коментатора и тексту коментара ако је та опција укључена.
- Коментари креирани овим API-јем и даље могу бити проверавани на спам ако је то потребно.
- Конфигурације као што је максимална дужина коментара, ако су подешене преко странице администрације правила прилагођавања (Customization Rule), важе и овде.

Минимални подаци потребни да се пошаљу и приказују у видгету коментара су следећи:

[inline-code-attrs-start title = 'Минимални пример POST cURL захтева за коментар'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

Реалнији пример захтева може изгледати овако:

[inline-code-attrs-start title = 'Пример POST cURL захтева за коментар'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
	/** Да ли коментар треба да се појави "у живо" за кориснике који гледају инстанце видгета коментара са истим urlId. НАПОМЕНА: Дуплира трошак кредита са 1 на 2. **/
    isLive?: 'true' | 'false'
    sendEmails?: 'true' | 'false'
    populateNotifications?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора POST захтева за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPostResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email';
    /** Укључено у случају неуспјеха. **/
    reason?: string
    /** Креирани коментар. **/
    comment?: Comment
    /** Повезани корисник, који можда већ постоји или можда није постојао. **/
    user?: User
}
[inline-code-end]

---