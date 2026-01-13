[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Glasove treba dohvatiti pomoću `urlId`.

### Vrste glasova

Postoje tri vrste glasova:

- Autentificirani glasovi, koji se primjenjuju na odgovarajući komentar. Možete ih stvoriti putem ovog API-ja.
- Autentificirani glasovi, koji su **na čekanju** verifikacije, i stoga još nisu primijenjeni na komentar. Oni se stvaraju kada korisnik koristi FastComments.com *prijava za glasanje* mehanizam.
- Anonimni glasovi, koji se primjenjuju na odgovarajući komentar. Oni se stvaraju zajedno s anonimnim komentiranjem.

Oni se vraćaju u odvojenim popisima u API-ju kako bi se smanjila zabuna.

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
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    /** Autorizirani, verificirani glasovi, primijenjeni na njihove odgovarajuće komentare. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonimni glasovi, primijenjeni na njihove odgovarajuće komentare. **/
    appliedAnonymousVotes: Vote[]
    /** Glasovi koji čekaju verifikaciju, još nisu primijenjeni na njihove odgovarajuće komentare. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Napomene o anonimnim glasovima

Napomena: anonimni glasovi stvoreni putem ovog API-ja pojavit će se u popisu `appliedAuthorizedVotes`. Smatraju se autoriziranim jer su stvoreni putem API-ja s API ključem.

Struktura `appliedAnonymousVotes` namijenjena je za glasove stvorene bez e-pošte, API ključa itd.