[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Omogućava dohvat glasova koje je korisnik ostavio na datom `urlId`. Prima `userId` koji može biti bilo koji FastComments.com korisnik ili `SSO User`.

Ovo je korisno ako želite prikazati da li je korisnik glasao za komentar. Pri dohvatanju komentara, jednostavno pozovite ovaj API istovremeno za korisnika sa istim `urlId`.

Ako koristite anonimno glasanje, umesto toga prosledite `anonUserId`.

[inline-code-attrs-start title = 'Primer cURL zahteva za glasove korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteva za glasove anonimnog korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Imajte na umu da će anonimni glasovi biti prikazani u listi `appliedAuthorizedVotes`. Oni se smatraju autorizovanim jer su kreirani preko API-ja uz API ključ.

[inline-code-attrs-start title = 'Struktura zahteva za glasove korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za glasove korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Uključeno pri grešci. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Uključeno pri grešci. **/
    reason?: string
    /** Autorizovani, verifikovani glasovi, primenjeni na odgovarajuće komentare. **/
    appliedAuthorizedVotes: Vote[]
    /** Glasovi koji čekaju verifikaciju, još nisu primenjeni na odgovarajuće komentare. **/
    pendingVotes: Vote[]
}
[inline-code-end]

---