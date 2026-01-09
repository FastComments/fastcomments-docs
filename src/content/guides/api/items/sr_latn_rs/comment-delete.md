[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API omogućava brisanje komentara.

Napomene:

- Ovaj API može ažurirati widget za komentare "uživo" ako je potrebno (ovo povećava `creditsCost` sa `1` na `2`).
- Ovaj API će obrisati sve podkomentare.

[inline-code-attrs-start title = 'Primer cURL zahteva za brisanje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahteva za brisanje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Korisnik koji vrši ažuriranje. Po potrebi se može koristiti za proveru da li može obrisati komentar.  **/
    contextUserId?: string
	/** Da li komentar treba da bude obrisan "uživo" za korisnike koji gledaju instance widgeta za komentare sa istim urlId. NAPOMENA: Udvostručuje trošak kredita sa 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** Uključeno pri neuspehu. **/
    reason?: string
}
[inline-code-end]

---