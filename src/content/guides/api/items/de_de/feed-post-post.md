[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Diese Route erstellt einen einzelnen `FeedPost`. Jeder Beitrag hat einen Autor, daher ist `fromUserId` erforderlich und muss die ID eines bestehenden FastComments- oder SSO-Benutzers
im Konto sein.

[inline-code-attrs-start title = 'FeedPost Erstellen cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'FeedPost Erstellen Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Schiebt den Beitrag zu Feeds, die gerade in einem Browser geöffnet sind. Standard ist false. **/
    isLive?: boolean
    /** Führt den Beitrag vor dem Speichern durch die Spam-Engine. Standard ist false. **/
    doSpamCheck?: boolean
    /** Überspringt die Wiederholungsinhalt-Prüfung, die im Rahmen von doSpamCheck ausgeführt wird. Standard ist false. **/
    skipDupCheck?: boolean
    /** Bis zu 256 Zeichen. An Live-Listener zurückgesendet, damit ein Client seine eigene Übertragung ignorieren kann. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Erforderlich. Eine FastComments- oder SSO-Benutzer-ID. **/
    fromUserId: string
    title?: string
    /** HTML. Beim Speichern bereinigt. **/
    contentHTML?: string
    /** Überschreibt den vom Benutzer übernommenen Anzeigenamen. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost Erstellen Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Bei Fehler enthalten. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Bei Fehler enthalten. **/
    reason?: string
    feedPost?: FeedPost; // Wir geben den vollständig erstellten Beitrag bei Erfolg zurück.
}
[inline-code-end]