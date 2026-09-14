[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Този маршрут създава един `FeedPost`. Всеки пост има автор, затова `fromUserId` е задължително и трябва да бъде идентификаторът на съществуващ FastComments или SSO потребител в акаунта.

[inline-code-attrs-start title = 'FeedPost Създаване cURL Пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Създаване Структура на заявка'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Изпраща поста към емисии, които са отворени в браузъра в момента. По подразбиране е false. **/
    isLive?: boolean
    /** Пуска поста през спам двигателя преди запазване. По подразбиране е false. **/
    doSpamCheck?: boolean
    /** Пропуска проверката за повторно съдържание, която се изпълнява като част от doSpamCheck. По подразбиране е false. **/
    skipDupCheck?: boolean
    /** До 256 знака. Отразява се към живи слушатели, за да може клиентът да игнорира собственото си излъчване. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Задължително. Идентификатор на FastComments или SSO потребител. **/
    fromUserId: string
    title?: string
    /** HTML. По време на запазване се почистват. **/
    contentHTML?: string
    /** Презаписва показваното име, взето от потребителя. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Създаване Структура на отговор'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Включено при неуспех. **/
    reason?: string
    feedPost?: FeedPost; // Връщаме целия създаден пост при успех.
}
[inline-code-end]