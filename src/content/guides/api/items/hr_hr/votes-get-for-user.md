[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Omogućava dohvaćanje glasova koje je korisnik ostavio za određeni `urlId`. Prima `userId` koji može biti bilo koji FastComments.com ili `SSO User`.

Ovo je korisno ako želite prikazati je li korisnik glasovao za komentar. Prilikom dohvaćanja komentara, jednostavno pozovite ovaj API istovremeno za korisnika sa istim `urlId`.

Ako koristite anonimno glasovanje, tada trebate umjesto toga proslijediti `anonUserId`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za glasove korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za anonimnog korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Imajte na umu da će se anonimni glasovi pojaviti u listi `appliedAuthorizedVotes`. Smatraju se autoriziranima budući da su stvoreni putem API-ja s API ključem.

[inline-code-attrs-start title = 'Struktura zahtjeva za glasove korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Uključeno pri neuspjehu. **/
    reason?: string
    /** Autorizirani, verificirani glasovi, primijenjeni na njihove odgovarajuće komentare. **/
    appliedAuthorizedVotes: Vote[]
    /** Glasovi koji čekaju verifikaciju, još nisu primijenjeni na odgovarajuće komentare. **/
    pendingVotes: Vote[]
}
[inline-code-end]

---