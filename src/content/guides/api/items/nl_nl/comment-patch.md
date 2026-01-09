[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt biedt de mogelijkheid om één reactie bij te werken.

Opmerkingen:

- Deze API kan de reactie-widget "live" bijwerken indien gewenst (dit verhoogt de basis `creditsCost` van `1` naar `2`).
  - Dit kan het migreren van reacties tussen pagina's "live" maken (door `urlId` te wijzigen).
  - Migraties kosten aanvullend `2` credits omdat pagina's vooraf worden berekend en dit CPU-intensief is.
- In tegenstelling tot de create API zal deze API GEEN gebruikersobjecten automatisch aanmaken in ons systeem als er een e‑mail wordt opgegeven.
- Reacties die via deze API worden bijgewerkt, kunnen indien gewenst nog steeds op spam worden gecontroleerd.
- Configuratie zoals maximale reactielengte, indien geconfigureerd via de Customization Rule admin page, is hier van toepassing.
- Om gebruikers toe te staan hun reactietekst bij te werken, kunt u eenvoudig `comment` in de request body opgeven. Wij genereren dan de resulterende `commentHTML`.
  - Als u zowel `comment` als `commentHTML` definieert, zullen wij de HTML niet automatisch genereren.
  - Als de gebruiker mentions of hashtags toevoegt in de nieuwe tekst, wordt dit nog steeds verwerkt zoals bij de `POST` API.
- Wanneer u `commenterEmail` op een reactie bijwerkt, is het het beste ook `userId` op te geven. Anders moet u ervoor zorgen dat de gebruiker met dit e‑mailadres tot uw tenant behoort, anders mislukt het verzoek.  


[inline-code-attrs-start title = 'Minimaal PATCH cURL-voorbeeld voor reactie'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van PATCH-verzoek voor reactie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** De gebruiker die de update uitvoert. Kan indien gewenst worden gebruikt om te controleren of zij de reactie kunnen bewerken.  **/
    contextUserId?: string
	/** Moeten we controleren of de nieuwe reactie op spam lijkt?  **/
    doSpamCheck?: 'true' | 'false'
	/** Of de reactie "live" moet verschijnen voor gebruikers die instanties van de reactie-widget met dezelfde urlId bekijken. OPMERKING: verdubbelt de creditkosten van 1 naar 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van PATCH-respons voor reactie'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

---