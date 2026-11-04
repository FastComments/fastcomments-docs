[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Изменяет опрос, не затрагивая его голоса. Используйте это, чтобы исправить опечатку в вопросе или варианте ответа, закрыть или заново открыть опрос, а также изменить то, кто может видеть, кто проголосовал.

Варианты адресуются по их `id`, и `PATCH` переименовывает только те, которые вы указали. Чтобы добавить, удалить или изменить порядок вариантов, отправьте полный список вариантов на `PUT /api/v1/polls/:commentId`: варианты, отправленные с их id, также сохраняют свои голоса.

Каждое поле необязательно, но должно быть указано хотя бы одно.

[inline-code-attrs-start title = 'Пример cURL-запроса PATCH для опроса'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL-запроса немедленного закрытия опроса'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса PATCH для опроса'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Переименовывает существующие варианты. Каждый указанный id уже должен быть в опросе. **/
    options?: { id: string, label: string }[] | null
    /** Дата в прошлом закрывает опрос немедленно. null заново открывает закрытый опрос. **/
    closesAt?: string | null
    /** 0 — анонимно, 1 — администраторы и модераторы, 2 — все. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа на PATCH для опроса'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Включается при ошибке. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Прочие примечания

- Указание id варианта, которого нет в опросе, приводит к ошибке `poll-invalid`, а не к тихому игнорированию.
- Названия вариантов должны оставаться уникальными в рамках опроса, с учётом вариантов, которые вы не изменяете.
- В отличие от создания опроса, `closesAt` здесь может быть в прошлом — именно так опрос закрывается немедленно.
- Приватность опроса можно сузить, но не расширить после того, как появились голоса.
- У опроса в заблокированном комментарии нельзя ничего изменить — будет ошибка `locked`.