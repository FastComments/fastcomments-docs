[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Цей маршрут створює один `FeedPost`. Кожен пост має автора, тому `fromUserId` є обов’язковим і має бути ідентифікатором існуючого користувача FastComments або SSO в обліковому записі.

[inline-code-attrs-start title = 'Приклад cURL створення FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запиту створення FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Надає пост у стрічки, які відкриті в браузері прямо зараз. За замовчуванням false. **/
    isLive?: boolean
    /** Запускає пост через спам‑двигун перед збереженням. За замовчуванням false. **/
    doSpamCheck?: boolean
    /** Пропускає перевірку повторюваного вмісту, яка виконується в рамках doSpamCheck. За замовчуванням false. **/
    skipDupCheck?: boolean
    /** До 256 символів. Повертається живим слухачам, щоб клієнт міг ігнорувати власне мовлення. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Обов’язково. Ідентифікатор користувача FastComments або SSO. **/
    fromUserId: string
    title?: string
    /** HTML. Очищено при збереженні. **/
    contentHTML?: string
    /** Перезаписує відображуване ім’я, отримане від користувача. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді створення FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Включено у випадку помилки. **/
    reason?: string
    feedPost?: FeedPost; // Ми повертаємо повний створений пост у випадку успіху.
}
[inline-code-end]