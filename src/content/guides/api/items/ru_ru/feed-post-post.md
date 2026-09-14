[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Этот маршрут создаёт один `FeedPost`. Каждый пост имеет автора, поэтому `fromUserId` обязателен и должен быть идентификатором существующего пользователя FastComments или SSO в аккаунте.

[inline-code-attrs-start title = 'Пример cURL для создания FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запроса создания FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Отправить пост в ленты, открытые в браузере прямо сейчас. По умолчанию false. **/
    isLive?: boolean
    /** Пропустить пост через спам‑движок перед сохранением. По умолчанию false. **/
    doSpamCheck?: boolean
    /** Пропустить проверку на повторяющийся контент, которая выполняется в рамках doSpamCheck. По умолчанию false. **/
    skipDupCheck?: boolean
    /** До 256 символов. Отправляется живым слушателям, чтобы клиент мог игнорировать собственную трансляцию. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Обязательно. Идентификатор пользователя FastComments или SSO. **/
    fromUserId: string
    title?: string
    /** HTML. Очищается при сохранении. **/
    contentHTML?: string
    /** Переопределяет отображаемое имя, полученное от пользователя. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа создания FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Включается при ошибке. **/
    reason?: string
    feedPost?: FeedPost; // Мы возвращаем полностью созданный пост при успехе.
}
[inline-code-end]