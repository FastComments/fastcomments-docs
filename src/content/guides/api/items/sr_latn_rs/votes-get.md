[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Glasovi se moraju preuzeti pomoću `urlId`.

### Tipovi glasova

Postoje tri tipa glasova:

- Authenticated Votes, koji se primenjuju na odgovarajući komentar. Možete ih kreirati putem ovog API-ja.
- Authenticated Votes, koji su **pending** verifikacije, i stoga još nisu primenjeni na komentar. Ovi se kreiraju kada korisnik koristi FastComments.com *login to vote* mehanizam.
- Anonymous Votes, koji se primenjuju na odgovarajući komentar. Oni se kreiraju zajedno sa anonimnim komentisanjem.

Oni se vraćaju u odvojenim listama u API-ju kako bi se smanjila zabuna.

[inline-code-attrs-start title = 'cURL primer za glasove'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za glasove'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za glasove'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    /** Autorizovani, verifikovani glasovi, primenjeni na odgovarajuće komentare. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonimni glasovi, primenjeni na odgovarajuće komentare. **/
    appliedAnonymousVotes: Vote[]
    /** Glasovi koji čekaju verifikaciju, još nisu primenjeni na odgovarajuće komentare. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Napomene o anonimnim glasovima

Imajte na umu da će anonimni glasovi kreirani putem ovog API-ja pojavljivati u listi `appliedAuthorizedVotes`. Smatraju se autorizovanim jer su kreirani putem API-ja uz API key.

Struktura `appliedAnonymousVotes` je za glasove kreirane bez email-a, API key-a, itd.