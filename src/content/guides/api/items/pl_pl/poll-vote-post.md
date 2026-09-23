[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Rejestruje głos w ankiecie.

Głosujący może oddać maksymalnie jeden głos w ankiecie. Wywołanie tego ponownie dla tego samego głosującego przenosi jego głos na nową opcję zamiast dodawać drugi, a głosowanie na opcję, którą już wybrał, nie robi nic.

Odpowiedź zawiera ankietę, więc otrzymujesz zaktualizowane liczniki bez dodatkowego żądania.

[inline-code-attrs-start title = 'Przykład cURL tworzenia PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Przykład cURL anonimowego tworzenia PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania tworzenia PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** One of userId or anonUserId is required. **/
    userId?: string
    anonUserId?: string
    /** The end user's IP, used for the anonymous rate limit. Defaults to the caller's IP. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi tworzenia PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Included on failure. **/
    reason?: string
    pollVote: PollVote
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### Głosy anonimowe

Ustaw `anonUserId` zamiast `userId`, aby zarejestrować głos dla osoby, która nie jest zalogowana. Ten identyfikator nie musi odpowiadać żadnemu użytkownikowi – po prostu identyfikuje sesję, więc ta sama osoba nie jest liczona podwójnie.

Anonimowe głosowanie musi być włączone dla Twojej witryny. Jeśli głosowanie jest ograniczone do zalogowanych użytkowników, głos z samym `anonUserId` kończy się niepowodzeniem z kodem `poll-login-required`.

Anonimowe głosy są również ograniczane pod względem liczby głosów na IP na ankietę, aby zapobiec nadużyciom, takim jak wielokrotne wypełnianie ankiety przez jedną osobę poprzez czyszczenie sesji. Prześlij `ip` końcowego użytkownika, aby limit dotyczył go, a nie Twojego serwera.

### Inne uwagi

- `userId` musi być użytkownikiem istniejącym w Twojej witrynie. Głosy dla użytkownika należącego do innej witryny są odrzucane.
- Głosowanie w zamkniętej ankiecie kończy się niepowodzeniem z kodem `poll-closed`.
- To API aktualizuje liczniki w ankiecie i przesyła je na żywo do podłączonych widgetów.