[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

Pozwala pobrać głosy oddane przez użytkownika na danym `urlId`. Przyjmuje `userId`, który może być dowolnym użytkownikiem FastComments.com lub `SSO User`.

Jest to przydatne, jeśli chcesz pokazać, czy użytkownik zagłosował na komentarz. Podczas pobierania komentarzy po prostu wywołaj to API jednocześnie dla użytkownika z tym samym `urlId`.

Jeśli używasz anonimowego głosowania, zamiast tego przekaż `anonUserId`.

[inline-code-attrs-start title = 'Przykład cURL: Głosy dla użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = 'Przykład cURL: Głosy dla anonimowego użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

Zauważ, że anonimowe głosy pojawią się na liście `appliedAuthorizedVotes`. Są one traktowane jako autoryzowane, ponieważ zostały utworzone za pomocą API z kluczem API.

[inline-code-attrs-start title = 'Struktura żądania: Głosy dla użytkownika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi: Głosy dla użytkownika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    /** Autoryzowane, zweryfikowane głosy, zastosowane do odpowiadających im komentarzy. **/
    appliedAuthorizedVotes: Vote[]
    /** Głosy oczekujące na weryfikację, jeszcze nie zastosowane do odpowiadających im komentarzy. **/
    pendingVotes: Vote[]
}
[inline-code-end]

---