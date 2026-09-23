[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Dołącza ankietę do istniejącego komentarza lub ustawia pełny stan ankiety, którą już posiada.

Treść jest pełną ankietą, a przesłane opcje stają się opcjami ankiety, w tej kolejności. Każda opcja
jest dopasowywana po jej `id`:

- Opcja wysłana z `id` istniejącej opcji zachowuje tę opcję i jej głosy. Jej etykieta i pozycja
  są aktualizowane do wartości, które przesłałeś.
- Opcja wysłana bez `id` jest dodawana, bez głosów.
- Istniejąca opcja, której nie uwzględnisz, jest usuwana wraz z oddanymi na nią głosami. `totalVotes` maleje o tę samą
  liczbę.

Aby dodać opcję, wyślij aktualne opcje z ich id oraz nową bez id. Aby usunąć jedną,
wyślij listę bez niej. Id opcji znajdują się w ankiecie zwróconej przez `GET /api/v1/polls/:commentId`.

Wysłanie żadnych id powoduje zastąpienie każdej opcji i usunięcie wszystkich głosów już oddanych w ankiecie. Jeśli ankieta ma
głosy, wymaga to `replaceVotes=true`, a bez tego API odpowiada `replace-votes-required`.

Inne pola również są zastępowane: pominięcie `closesAt`, `privacy` lub `requireVoteToSeeResults` resetuje je do
wartości domyślnej. Aby zmienić pojedyncze pole i pozostawić pozostałe bez zmian, użyj `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Przykład cURL dla Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Wymagane, aby nie zachować żadnych istniejących id opcji, gdy ankieta ma głosy, ponieważ to usuwa je wszystkie. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** Id istniejącej opcji, aby zachować ją i jej głosy. Pomiń, aby dodać nową opcję. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** Pełna, uporządkowana lista. Pominięte istniejące opcje są usuwane wraz z ich głosami. **/
    options: PollPutOption[]
    /** Musi być w przyszłości, gdy komentarz nie ma jeszcze ankiety. Pomiń dla ankiety, która pozostaje otwarta. **/
    closesAt?: string | null
    /** 0 anonimowi (domyślnie), 1 administratorzy i moderatorzy, 2 wszyscy. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Inne uwagi

- `id`, które nie znajduje się w ankiecie, lub podane dwukrotnie, powoduje błąd `poll-invalid`. Komentarz bez
  ankiety nie ma jeszcze id opcji, więc każda przesłana do niego opcja musi nie zawierać `id`.
- Prywatność ankiety może być ograniczona, ale nie może być rozszerzona po tym, jak otrzyma głosy.
- To API respektuje ustawienia Twojej witryny. Jeśli ankiety nie są włączone dla witryny lub strony, zwraca błąd
  `polls-disabled`.
- Zablokowany komentarz nie może mieć zmienionej ankiety i zwraca błąd `locked`.
- Połączone widżety są aktualizowane na żywo, więc widzowie widzą nową ankietę bez odświeżania.