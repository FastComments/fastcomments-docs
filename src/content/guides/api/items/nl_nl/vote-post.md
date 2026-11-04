[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om een enkele geautoriseerde `Vote` toe te voegen. Stemmen kunnen `up` (+1) of `down` (-1) zijn.

[inline-code-attrs-start title = 'Vote Aanmaken cURL Voorbeeld'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Anonieme Vote Aanmaken cURL Voorbeeld'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote Aanmaken Request Structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Vote Aanmaken Response Structuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Anonieme Stemmen Aanmaken

Anonieme stemmen kunnen worden aangemaakt door `anonUserId` in te stellen in de query params in plaats van `userId`.

Deze id hoeft niet te corresponderen met een gebruikersobject ergens (vandaar anoniem). Het is simpelweg een identificatie
voor de sessie, zodat je stemmen opnieuw kunt ophalen in dezelfde sessie, om te controleren of een reactie
al gestemd is.

Als je zoiets als "anonieme sessies" niet hebt zoals FastComments dat heeft - dan kun je dit
simpelweg instellen op een willekeurige ID, zoals een UUID (hoewel we kleinere identificaties waarderen om ruimte te besparen).

### Overige Opmerkingen

- Deze API volgt de instellingen op tenant-niveau. Als je bijvoorbeeld stemmen uitschakelt voor een bepaalde pagina en je probeert via de API een stem te maken, zal dit mislukken met foutcode `voting-disabled`.
- Deze API is standaard live.
- Deze API werkt de `votes` van de bijbehorende `Comment` bij.