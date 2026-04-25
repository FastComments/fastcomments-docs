[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućava brisanje komentara.

Napomene:

- Ovaj API može ažurirati widget za komentare "uživo" ako je željeno (ovo povećava `creditsCost` sa `1` na `2`).
- Ovaj API će izbrisati sve podkomentare.
- Ako je ciljani komentar zaključan (`isLocked: true`), zahtjev se odbija sa `code: 'locked'`. Prvo otključajte komentar, zatim obrišite.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za brisanje komentara'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura zahtjeva za brisanje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Korisnik koji vrši ažuriranje. Po potrebi se može koristiti za provjeru da li mogu obrisati komentar.  **/
    contextUserId?: string
	/** Da li komentar treba biti obrisan "uživo" za korisnike koji gledaju instance widgeta za komentare sa istim urlId. NAPOMENA: Udvostručuje trošak kredita sa 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Uključeno pri neuspjehu. **/
    reason?: string
}
[inline-code-end]

---