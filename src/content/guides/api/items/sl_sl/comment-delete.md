[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ta končna točka API-ja omogoča brisanje komentarja.

Opombe:

- Ta API lahko po želji posodobi widget komentarjev "v živo" (to poveča `creditsCost` s `1` na `2`).
- Ta API bo izbrisal vse otroške komentarje.
- Če je ciljni komentar zaklenjen (`isLocked: true`), je zahteva zavrnjena z `code: 'locked'`. Najprej odklenite komentar, nato ga izbrišite.

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
	/** Uporabnik, ki izvaja posodobitev. Po želji se lahko uporabi za preverjanje, ali lahko izbriše komentar.  **/
    contextUserId?: string
	/** Ali naj bo komentar izbrisan "v živo" za uporabnike, ki gledajo primere widgeta komentarjev z istim urlId. OPOMBA: Podvoji strošek kreditov s 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje komentarja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Vključeno v primeru neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Vključeno v primeru neuspeha. **/
    reason?: string
}
[inline-code-end]

---