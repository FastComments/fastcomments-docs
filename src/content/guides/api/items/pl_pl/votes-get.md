[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

Głosy muszą być pobierane za pomocą `urlId`.

### Typy głosów

Istnieją trzy rodzaje głosów:

- Uwierzytelnione głosy, które są przypisane do odpowiadającego komentarza. Możesz je tworzyć za pomocą tego API.
- Uwierzytelnione głosy, które są **w oczekiwaniu** na weryfikację, i w związku z tym nie zostały jeszcze zastosowane do komentarza. Są tworzone, gdy użytkownik użyje mechanizmu FastComments.com *login to vote*.
- Anonimowe głosy, które są przypisane do odpowiadającego komentarza. Są tworzone wraz z anonimowym komentowaniem.

Są one zwracane w oddzielnych listach w API, aby zmniejszyć zamieszanie.

[inline-code-attrs-start title = 'Przykład żądania cURL dla głosów'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania głosów'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi głosów'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    /** Autoryzowane, zweryfikowane głosy, przypisane do odpowiadających komentarzy. **/
    appliedAuthorizedVotes: Vote[]
    /** Anonimowe głosy, przypisane do odpowiadających komentarzy. **/
    appliedAnonymousVotes: Vote[]
    /** Głosy oczekujące na weryfikację, jeszcze nie zastosowane do odpowiadających komentarzy. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### Uwagi dotyczące anonimowych głosów

Zauważ, że anonimowe głosy utworzone za pomocą tego API będą pojawiać się na liście `appliedAuthorizedVotes`. Są one uznawane za autoryzowane, ponieważ zostały utworzone przez API z wykorzystaniem klucza API.

Struktura `appliedAnonymousVotes` dotyczy głosów utworzonych bez adresu e-mail, klucza API itp.

---