[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Ermöglicht das Abrufen von Abstimmungen, die von einem Benutzer für eine bestimmte `urlId` hinterlassen wurden. Nimmt eine `userId` entgegen, die jeder FastComments.com- oder `SSO-Benutzer` sein kann.

Dies ist nützlich, wenn Sie zeigen möchten, ob ein Benutzer für einen Kommentar abgestimmt hat. Wenn Sie Kommentare abrufen, rufen Sie diese API einfach gleichzeitig für den Benutzer mit der
gleichen `urlId` auf.

Wenn Sie anonyme Abstimmungen verwenden, möchten Sie stattdessen `anonUserId` übergeben.

[inline-code-attrs-start title = 'Votes für Benutzer cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Votes für anonymen Benutzer cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Beachten Sie, dass anonyme Abstimmungen in der `appliedAuthorizedVotes`-Liste erscheinen. Sie gelten als autorisiert, da sie über die API mit einem API-Schlüssel erstellt wurden.

[inline-code-attrs-start title = 'Votes für Benutzer Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Votes für Benutzer Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Included on failure. **/
    reason?: string
    /** Authorized, verified votes, applied to their corresponding comments. **/
    appliedAuthorizedVotes: Vote[]
    /** Votes pending verification, not yet applied to their corresponding comments. **/
    pendingVotes: Vote[]
}
[inline-code-end]
