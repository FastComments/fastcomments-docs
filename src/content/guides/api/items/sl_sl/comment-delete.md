[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ta API endpoint omogoča brisanje komentarja.

Opombe:

- Ta API lahko po želji posodobi pripomoček za komentarje "v živo" (to poveča `creditsCost` s `1` na `2`).
- Ta API bo izbrisal vse podrejene komentarje.

[inline-code-attrs-start title = 'Primer cURL zahteve za brisanje komentarja'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahteve za brisanje komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Uporabnik, ki izvaja posodobitev. Po želji se lahko uporabi za preverjanje, ali lahko izbriše komentar. **/
    contextUserId?: string
	/** Ali naj bo komentar izbrisan "v živo" za uporabnike, ki si ogledujejo primerke pripomočka za komentarje z enakim urlId. OPOMBA: Podvoji strošek kreditov s 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Vključeno ob napaki. **/
    reason?: string
}
[inline-code-end]

---