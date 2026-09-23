[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Edytuje ankietę bez zakłócania jej głosów. Użyj tego, aby naprawić literówkę w pytaniu lub opcji, zamknąć lub
ponownie otworzyć ankietę, lub zmienić, kto może zobaczyć, kto zagłosował.

Opcje są identyfikowane przez ich `id`, a `PATCH` zmienia ich etykiety na podane przez Ciebie. Aby dodać, usunąć lub zmienić kolejność
opcji, wyślij pełną listę opcji do `PUT /api/v1/polls/:commentId`: opcje, które wyślesz wraz z ich identyfikatorami, zachowają
swoje głosy.

Każde pole jest opcjonalne, ale przynajmniej jedno musi być podane.

[inline-code-attrs-start title = 'Przykład cURL Patch Ankiety'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Przykład cURL Zamknięcia Ankiety'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Żądania Patch Ankiety'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Zmieniane są etykiety istniejących opcji. Każde podane id musi już znajdować się w ankiecie. **/
    options?: { id: string, label: string }[] | null
    /** Data w przeszłości zamyka ankietę natychmiast. null ponownie otwiera zamkniętą ankietę. **/
    closesAt?: string | null
    /** 0 anonimowi, 1 administratorzy i moderatorzy, 2 wszyscy. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura Odpowiedzi Patch Ankiety'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Inne uwagi

- Nadanie identyfikatora opcji, który nie istnieje w ankiecie, kończy się błędem `poll-invalid` zamiast cichego braku działania.
- Etykiety muszą pozostać unikalne w ankiecie, uwzględniając opcje, których nie zmieniasz.
- W przeciwieństwie do tworzenia ankiety, `closesAt` może być w przeszłości – tak zamykasz ankietę natychmiast.
- Prywatność ankiety może być ograniczona, ale nie może być rozszerzona po otrzymaniu głosów.
- Zablokowany komentarz nie może mieć zmienionej ankiety i zwraca błąd `locked`.

---