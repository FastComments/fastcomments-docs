---
 [api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om één geautoriseerde `Vote` toe te voegen. Votes kunnen `up` (+1) of `down` (-1) zijn.

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor het aanmaken van een stem'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor het aanmaken van een anonieme stem'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het verzoek voor het aanmaken van een stem'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structuur van de respons voor het aanmaken van een stem'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Opgenomen bij een fout. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Creating Anonymous Votes

Anonieme stemmen kunnen worden aangemaakt door `anonUserId` in de queryparameters te zetten in plaats van `userId`.

Deze id hoeft nergens overeen te komen met een gebruikersobject (vandaar anoniem). Het is simpelweg een identificator
voor de sessie, zodat je stemmen opnieuw kunt ophalen in dezelfde sessie, om te controleren of op een reactie
is gestemd.

Als je niet zoiets hebt als "anonymous sessions" zoals FastComments - je kunt dit eenvoudig
instellen op een willekeurige ID, zoals een UUID (hoewel we kleinere identificatoren waarderen om ruimte te besparen).

### Other Notes

- Deze API houdt zich aan tenant-niveau instellingen. Bijvoorbeeld, als je stemmen uitschakelt voor een bepaalde pagina, en je probeert via de API een stem aan te maken, zal dit falen met foutcode `voting-disabled`.
- Deze API is standaard actief.
- Deze API zal de `votes` van de overeenkomstige `Comment` bijwerken.

---