[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Registrerer en stemme i en afstemning.

En vælger kan højst have én stemme pr. afstemning. Hvis du kalder dette igen for den samme vælger, flyttes deres stemme til den nye mulighed i stedet for at tilføje en anden, og at stemme på den mulighed, de allerede har valgt, gør intet.

Svaret inkluderer afstemningen, så du får de opdaterede optællinger uden en ekstra forespørgsel.

[inline-code-attrs-start title = 'PollVote Opret cURL Eksempel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Anonym PollVote Opret cURL Eksempel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVote Opret Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'PollVote Opret Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Anonyme Stemmer

Sæt `anonUserId` i stedet for `userId` for at registrere en stemme for en, der ikke er logget ind. Det id behøver ikke svare til en bruger nogen steder – det identificerer blot sessionen, så den samme person ikke tælles to gange.

Anonym afstemning skal være aktiveret for dit site. Hvis afstemning er begrænset til loggede brugere, fejler en stemme med kun et `anonUserId` med `poll-login-required`.

Anonyme stemmer er også hastighedsbegrænsede pr. IP pr. afstemning for at forhindre, at én person fylder en afstemning ved at rydde deres session. Send slutbrugerens `ip`, så begrænsningen gælder dem i stedet for din server.

### Andre Bemærkninger

- En `userId` skal være en bruger, der findes på dit site. Stemmer for en bruger, der tilhører et andet site, afvises.
- Afstemning i en lukket afstemning fejler med `poll-closed`.
- Dette API opdaterer optællingerne på afstemningen og sender dem live til tilsluttede widgets.