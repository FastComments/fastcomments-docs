[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Dit API-eindpunt biedt de mogelijkheid om een reactie te verwijderen.

Opmerkingen:

- Deze API kan de reactie-widget "live" bijwerken indien gewenst (dit verhoogt `creditsCost` van `1` naar `2`).
- Deze API zal alle onderliggende reacties verwijderen.
- Als de doelreactie is vergrendeld (`isLocked: true`), wordt het verzoek afgewezen met `code: 'locked'`. Ontgrendel eerst de reactie en verwijder deze daarna.

[inline-code-attrs-start title = 'Comment DELETE cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Comment DELETE Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** De gebruiker die de update uitvoert. Kan, indien gewenst, worden gebruikt om te controleren of deze de reactie kan verwijderen.  **/
    contextUserId?: string
	/** Of de reactie "live" moet worden verwijderd voor gebruikers die instanties van de reactie-widget met dezelfde urlId bekijken. OPMERKING: Verdubbelt de credits-kost van 1 naar 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment DELETE Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

---