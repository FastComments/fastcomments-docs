[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at tilføje en enkelt autoriseret `Vote`. Stemmer kan være `up` (+1) eller `down` (-1).

[inline-code-attrs-start title = 'Vote Oprettelse cURL Eksempel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Anonym Vote Oprettelse cURL Eksempel'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote Oprettelse Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote Oprettelse Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Oprettelse af Anonyme Stemmer

Anonyme stemmer kan oprettes ved at sætte `anonUserId` i forespørgselsparametrene i stedet for `userId`.

Dette id behøver ikke at svare til et brugerobjekt nogen steder (deraf anonymt). Det er simpelthen en identifikator
for sessionen, så du kan hente stemmer igen i samme session for at tjekke, om der er stemt på en kommentar.

Hvis du ikke har noget som "anonyme sessioner" som FastComments har - kan du blot
sætte dette til et tilfældigt ID, som en UUID (selvom vi sætter pris på mindre identifikatorer for at spare plads).

### Andre Bemærkninger

- Dette API overholder tenant-niveau indstillinger. For eksempel, hvis du deaktiverer afstemning for en given side, og du forsøger at oprette en stemme via API'et, vil det fejle med fejlkode `voting-disabled`.
- Dette API er live som standard.
- Dette API vil opdatere `votes` for den tilsvarende `Comment`.
