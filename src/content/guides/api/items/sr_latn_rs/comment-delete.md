[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava brisanje komentara.

Napomene:

- Ovaj API može ažurirati widget komentara uživo ako je potrebno (ovo povećava `creditsCost` sa `1` na `2`).
- Ovaj API će obrisati sve podkomentare.
- Ako je ciljani komentar zaključan (`isLocked: true`), zahtev se odbija sa `code: 'locked'`. Prvo otključajte komentar, pa zatim obrišite.

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
	/** Da li komentar treba biti obrisan uživo za korisnike koji gledaju instance widgeta komentara sa istim urlId. NAPOMENA: Udupljuje trošak kredita sa 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Uključeno pri neuspehu. **/
    reason?: string
}
[inline-code-end]