[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Deze API-endpoint biedt de mogelijkheid om een reactie te verwijderen.

Opmerkingen:

- Deze API kan, indien gewenst, de comment-widget "live" bijwerken (dit verhoogt `creditsCost` van `1` naar `2`).
- Deze API verwijdert alle onderliggende reacties.

[inline-code-attrs-start title = 'Comment DELETE cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Structuur van Comment DELETE-verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** De gebruiker die de update uitvoert. Kan, indien gewenst, worden gebruikt om te controleren of de gebruiker de reactie mag verwijderen.  **/
    contextUserId?: string
	/** Of de reactie "live" moet worden verwijderd voor gebruikers die instanties van de comment-widget met dezelfde urlId bekijken. OPMERKING: verdubbelt de credits-kosten van 1 naar 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van Comment DELETE-respons'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

---