[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Овај рут креира један `FeedPost`. Сваки пост има аутора, па је `fromUserId` обавезан и мора бити ИД постојећег FastComments или SSO корисника на налогу.

[inline-code-attrs-start title = 'FeedPost Креирање cURL Пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'FeedPost Креирање Захтевне Структуре'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Пошаљи пост у фидове који су тренутно отворени у прегледачу. Подразумевано је false. **/
    isLive?: boolean
    /** Покрени пост кроз спам мотор пре чувања. Подразумевано је false. **/
    doSpamCheck?: boolean
    /** Прескочи проверу поновљеног садржаја која се извршава као део doSpamCheck. Подразумевано је false. **/
    skipDupCheck?: boolean
    /** До 256 карактера. Одбија се живим слушаоцима тако да клијент може игнорисати своје сопствено емитовање. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Обавезно. ИД FastComments или SSO корисника. **/
    fromUserId: string
    title?: string
    /** HTML. Санитизовано при чувању. **/
    contentHTML?: string
    /** Преписује име за приказ које је узето од корисника. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Креирање Одговорне Структуре'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Укључено при неуспеху. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Укључено при неуспеху. **/
    reason?: string
    feedPost?: FeedPost; // Враћамо комплетан креирани пост при успеху.
}
[inline-code-end]