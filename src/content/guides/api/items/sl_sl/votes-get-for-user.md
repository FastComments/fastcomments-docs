[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Omogoča pridobivanje glasov, ki jih je uporabnik oddal za določen `urlId`. Sprejme `userId`, ki je lahko katerikoli FastComments.com ali `SSO User`.

To je uporabno, če želite prikazati, ali je uporabnik glasoval pri komentarju. Pri pridobivanju komentarjev preprosto pokličite to API istočasno za uporabnika z
istim `urlId`.

Če uporabljate anonimno glasovanje, boste namesto tega želeli posredovati `anonUserId`.

[inline-code-attrs-start title = 'Primer cURL zahteve za glasove uporabnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteve za anonimnega uporabnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Upoštevajte, da se bodo anonimni glasovi pojavili v seznamu `appliedAuthorizedVotes`. Štejemo jih za pooblaščene, saj so bili ustvarjeni preko API z API ključem.

[inline-code-attrs-start title = 'Struktura zahteve za glasove uporabnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za glasove uporabnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Vključeno ob neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Vključeno ob neuspehu. **/
    reason?: string
    /** Pooblaščeni, preverjeni glasovi, uporabljeni pri pripadajočih komentarjih. **/
    appliedAuthorizedVotes: Vote[]
    /** Glasovi, ki čakajo na preverjanje, še niso uporabljeni pri pripadajočih komentarjih. **/
    pendingVotes: Vote[]
}
[inline-code-end]

---