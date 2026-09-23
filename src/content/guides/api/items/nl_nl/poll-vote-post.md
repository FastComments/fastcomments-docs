[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Registreert een stem op een poll.

Een kiezer heeft maximaal één stem per poll. Als je dit opnieuw aanroept voor dezelfde kiezer, wordt hun stem verplaatst naar de nieuwe optie in plaats van een tweede stem toe te voegen, en stemmen op de optie die ze al gekozen hebben doet niets.

De respons bevat de poll, zodat je de bijgewerkte tellingen krijgt zonder een tweede verzoek.

[inline-code-attrs-start title = 'PollVote Aanmaak cURL Voorbeeld'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Anonieme PollVote Aanmaak cURL Voorbeeld'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Aanmaak Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** One of userId or anonUserId is required. **/
    userId?: string
    anonUserId?: string
    /** The end user's IP, used for the anonymous rate limit. Defaults to the caller's IP. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote Aanmaak Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Included on failure. **/
    reason?: string
    pollVote: PollVote
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Anonieme stemmen

Stel `anonUserId` in plaats van `userId` in om een stem te registreren voor iemand die niet is ingelogd. Die id hoeft nergens overeen te komen met een gebruiker – hij identificeert alleen de sessie, zodat dezelfde persoon niet twee keer wordt geteld.

Anoniem stemmen moet voor je site zijn ingeschakeld. Als stemmen beperkt is tot ingelogde gebruikers, mislukt een stem met alleen een `anonUserId` met `poll-login-required`.

Anonieme stemmen zijn ook per IP per poll rate‑gelimiteerd, om te voorkomen dat één persoon een poll vult door hun sessie te wissen. Stuur de `ip` van de eindgebruiker zodat de limiet op hen wordt toegepast in plaats van op je server.

### Andere opmerkingen

- Een `userId` moet een gebruiker zijn die bestaat op je site. Stemmen voor een gebruiker die tot een andere site behoort, worden afgewezen.
- Stemmen op een gesloten poll mislukt met `poll-closed`.
- Deze API werkt de tellingen op de poll bij en stuurt ze live naar verbonden widgets.