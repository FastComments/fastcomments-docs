[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Glasove je treba pridobiti po `urlId`.

### Vrste glasov

Obstajajo tri vrste glasov:

- Avtentificirani glasovi, ki se uporabljajo za ustrezen komentar. Te lahko ustvarite preko tega API-ja.
- Avtentificirani glasovi, ki so **v postopku preverjanja**, in zato še niso uporabljeni za komentar. Ti se ustvarijo, ko uporabnik uporabi mehanizem FastComments.com *prijava za glasovanje*.
- Anonimni glasovi, ki se uporabljajo za ustrezen komentar. Ti se ustvarijo skupaj z anonimnim komentiranjem.

V API-ju so ti vrnjeni v ločenih seznamih, da se zmanjša zmeda.

[inline-code-attrs-start title = 'Primer cURL zahteve za glasove'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za glasove'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Vključeno ob napaki. **/
    reason?: string
    /** Pooblaščeni, preverjeni glasovi, uporabljeni za ustrezne komentarje. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonimni glasovi, uporabljeni za ustrezne komentarje. **/
    appliedAnonymousVotes: Vote[]
    /** Glasovi v postopku preverjanja, še niso uporabljeni za ustrezne komentarje. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Opombe o anonimnih glasovih

Upoštevajte, da se bodo anonimni glasovi, ustvarjeni preko tega API-ja, pojavili v seznamu `appliedAuthorizedVotes`. Štejemo jih za pooblaščene, saj so bili ustvarjeni preko API-ja z API ključem.

Struktura `appliedAnonymousVotes` je za glasove, ustvarjene brez e-pošte, API ključa itd.