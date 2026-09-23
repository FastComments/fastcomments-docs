[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Wyświetla poszczególne głosy stojące za liczbami jednej ankiety, najstarsze najpierw. Jeden kredyt za każde 100 zwróconych głosów.

Ankieta należy do komentarza, więc głosy są odczytywane jedną ankietę na raz i wymaga się `commentId`. Można dodatkowo zawęzić
przy pomocy `voterId`, aby sprawdzić, jak zagłosowała konkretna osoba, lub przy pomocy `optionId`, aby wylistować wszystkich, którzy wybrali daną opcję.

Maksymalnie 1000 głosów jest zwracanych na jedno wywołanie. Użyj `skip`, aby stronicować dalej.

Ustawienie `privacy` ankiety jest respektowane: głosy w anonimowej ankiecie nie mogą być odczytane, a żądanie kończy się niepowodzeniem
z kodem `poll-anonymous`. Zobacz strukturę `PollVote` po szczegóły.

[inline-code-attrs-start title = 'Przykład cURL Pobierania PollVotes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi PollVotes Get'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### Zliczanie głosów na opcję

Nie musisz sumować ich, aby uzyskać wyniki – ankieta posiada własne liczniki. Zamiast tego odczytaj ankietę za pomocą
`GET /api/v1/polls/:commentId`, i użyj tego API, gdy potrzebujesz wiedzieć, kto zagłosował.

### Każda ankieta na stronie

Nie ma listy głosów obejmującej całą stronę. Aby sporządzić raport dla całej strony, pobierz jej komentarze za pomocą
`GET /api/v1/comments`, które zwracają ankietę każdego komentarza oraz jej liczniki, a następnie odczytaj głosy dla
ankiet, które Cię interesują.