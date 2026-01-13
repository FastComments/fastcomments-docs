[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Abstimmungen müssen nach `urlId` abgerufen werden.

### Arten von Abstimmungen

Es gibt drei Arten von Abstimmungen:

- Authentifizierte Abstimmungen, die auf den entsprechenden Kommentar angewendet werden. Diese können Sie über diese API erstellen.
- Authentifizierte Abstimmungen, die auf Verifizierung **warten**, und daher noch nicht auf den Kommentar angewendet wurden. Diese werden erstellt, wenn ein Benutzer den FastComments.com-*Login-zum-Abstimmen*-Mechanismus verwendet.
- Anonyme Abstimmungen, die auf den entsprechenden Kommentar angewendet werden. Diese werden zusammen mit anonymen Kommentaren erstellt.

Diese werden in separaten Listen in der API zurückgegeben, um Verwirrung zu vermeiden.

[inline-code-attrs-start title = 'Votes cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Votes Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Votes Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonymous votes, applied to their corresponding comments. **/
    appliedAnonymousVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Hinweise zu anonymen Abstimmungen

Beachten Sie, dass anonyme Abstimmungen, die über diese API erstellt werden, in der `appliedAuthorizedVotes`-Liste erscheinen. Sie gelten als autorisiert, da sie über die API mit einem API-Schlüssel erstellt wurden.

Die `appliedAnonymousVotes`-Struktur ist für Abstimmungen, die ohne E-Mail, API-Schlüssel usw. erstellt wurden.
