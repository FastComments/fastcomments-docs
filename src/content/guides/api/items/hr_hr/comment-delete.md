[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Ovaj API endpoint omogućuje brisanje komentara.

Napomene:

- Ovaj API može ažurirati widget komentara "uživo" ako se želi (ovo povećava `creditsCost` s `1` na `2`).
- Ovaj API će obrisati sve podkomentare.
- Ako je ciljani komentar zaključan (`isLocked: true`), zahtjev se odbija s `code: 'locked'`. Prvo otključajte komentar, zatim ga obrišite.

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
	/** Korisnik koji izvodi ažuriranje. Po potrebi se može koristiti za provjeru ima li pravo obrisati komentar. **/
    contextUserId?: string
	/** Treba li se komentar obrisati "uživo" korisnicima koji pregledavaju instance widgeta komentara s istim urlId. NAPOMENA: Udvostručuje trošak kredita s 1 na 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za brisanje komentara'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]