[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt biedt de mogelijkheid om een enkele opmerking bij te werken.

Opmerkingen:

- Deze API kan het reacties-widget "live" bijwerken indien gewenst (dit verhoogt de basis `creditsCost` van `1` naar `2`).
  - Hiermee kunnen migraties van opmerkingen tussen pagina's "live" worden gemaakt (door `urlId` te wijzigen).
  - Migraties kosten een extra `2` credits omdat pagina's worden voorgecalculeerd en dit CPU-intensief is.
- In tegenstelling tot de create-API zal deze API GEEN gebruikersobjecten automatisch aanmaken in ons systeem als een e-mail wordt opgegeven.
- Via deze API bijgewerkte opmerkingen kunnen indien gewenst nog steeds op spam worden gecontroleerd.
- Configuratie zoals maximale commentaarlengte, indien geconfigureerd via de Customization Rule-beheerpagina, is hier van toepassing.
- Om gebruikers toe te staan hun commentaartekst bij te werken, kunt u gewoon `comment` opgeven in de request body. Wij zullen de resulterende `commentHTML` genereren.
  - Als u zowel `comment` als `commentHTML` opgeeft, zullen we de HTML niet automatisch genereren.
  - Als de gebruiker mentions of hashtags aan zijn nieuwe tekst toevoegt, wordt dit nog steeds verwerkt zoals bij de `POST`-API.
- Wanneer u `commenterEmail` bijwerkt op een opmerking, is het het beste ook `userId` op te geven. Anders moet u ervoor zorgen dat de gebruiker met dit e-mailadres tot uw tenant behoort, anders zal het verzoek mislukken.  
- Als de doelopmerking vergrendeld is (`isLocked: true`), wordt het verzoek afgewezen met `code: 'locked'`. Ontgrendel de opmerking eerst, werk deze bij, en vergrendel opnieuw indien gewenst.


[inline-code-attrs-start title = 'Minimaal PATCH cURL-voorbeeld voor Opmerking'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van PATCH-verzoek voor Opmerking'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** De gebruiker die de update uitvoert. Indien gewenst kan dit worden gebruikt om te controleren of zij de opmerking mogen bewerken.  **/
    contextUserId?: string
	/** Moeten we controleren of de nieuwe opmerking op spam lijkt?  **/
    doSpamCheck?: 'true' | 'false'
	/** Of de opmerking "live" moet verschijnen voor gebruikers die instanties van het comment-widget met dezelfde urlId bekijken. OPMERKING: Verdubbelt de creditkost van 1 naar 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van PATCH-respons voor Opmerking'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]