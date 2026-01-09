[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Glasovi se moraju dohvatiti pomoću `urlId`.

### Vrste glasova

Postoje tri vrste glasova:

- Autentifikovani glasovi, koji se primjenjuju na odgovarajući komentar. Možete ih kreirati putem ovog API-ja.
- Autentifikovani glasovi, koji su **na čekanju** verifikacije, i stoga još nisu primijenjeni na komentar. Ovi se kreiraju kada korisnik koristi FastComments.com mehanizam *prijave za glasanje*.
- Anonimni glasovi, koji se primjenjuju na odgovarajući komentar. Oni se kreiraju zajedno sa anonimnim komentiranjem.

Oni se vraćaju u odvojenim listama u API-ju kako bi se smanjila zabuna.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za glasove'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Uključeno u slučaju greške. **/
    reason?: string
    /** Autorizovani, verifikovani glasovi, primijenjeni na odgovarajuće komentare. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonimni glasovi, primijenjeni na odgovarajuće komentare. **/
    appliedAnonymousVotes: Vote[]
    /** Glasovi koji čekaju verifikaciju, još nisu primijenjeni na odgovarajuće komentare. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Napomene o anonimnim glasovima

Imajte na umu da će anonimni glasovi kreirani putem ovog API-ja pojaviti u listi `appliedAuthorizedVotes`. Smatraju se autorizovanim jer su kreirani putem API-ja sa API key-om.

Struktura `appliedAnonymousVotes` je za glasove kreirane bez emaila, API key-a, itd.