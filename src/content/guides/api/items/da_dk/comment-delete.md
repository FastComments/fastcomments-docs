[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Dette API-endpoint giver mulighed for at slette en kommentar.

Bemærkninger:

- Dette API kan opdatere kommentar-widgeten "live", hvis ønsket (det øger `creditsCost` fra `1` til `2`).
- Dette API vil slette alle underordnede kommentarer.
- Hvis den målrettede kommentar er låst (`isLocked: true`), afvises anmodningen med `code: 'locked'`. Lås kommentaren op først, og slet derefter.

[inline-code-attrs-start title = 'Kommentar DELETE cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Kommentar DELETE Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Brugeren, der udfører opdateringen. Kan efter ønske bruges til at kontrollere, at de kan slette kommentaren.  **/
    contextUserId?: string
	/** Hvorvidt kommentaren skal slettes "live" for brugere, der ser instanser af kommentar-widget'en med samme urlId. BEMÆRK: Dobbler kreditomkostningen fra 1 til 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Kommentar DELETE Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Inkluderet ved fejl. **/
    reason?: string
}
[inline-code-end]