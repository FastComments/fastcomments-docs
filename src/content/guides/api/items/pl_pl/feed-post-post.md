[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Ta trasa tworzy pojedynczy `FeedPost`. Każdy post ma autora, więc `fromUserId` jest wymagane i musi być identyfikatorem istniejącego użytkownika FastComments lub SSO na koncie.

[inline-code-attrs-start title = 'Przykład cURL tworzenia FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura żądania tworzenia FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Wysyła post do kanałów otwartych w przeglądarce w tej chwili. Domyślnie false. **/
    isLive?: boolean
    /** Uruchamia post przez silnik antyspamowy przed zapisaniem. Domyślnie false. **/
    doSpamCheck?: boolean
    /** Pomija sprawdzanie powtarzającej się treści, które jest wykonywane w ramach doSpamCheck. Domyślnie false. **/
    skipDupCheck?: boolean
    /** Do 256 znaków. Echoowane do słuchaczy na żywo, aby klient mógł zignorować własną transmisję. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Wymagane. Identyfikator użytkownika FastComments lub SSO. **/
    fromUserId: string
    title?: string
    /** HTML. Oczyszczane przy zapisie. **/
    contentHTML?: string
    /** Zastępuje wyświetlaną nazwę pobraną od użytkownika. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi tworzenia FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    feedPost?: FeedPost; // Zwracamy kompletny utworzony post w przypadku sukcesu.
}
[inline-code-end]