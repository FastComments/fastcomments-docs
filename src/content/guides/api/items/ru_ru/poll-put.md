[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Прикрепляет опрос к существующему комментарию или задаёт полное состояние уже существующего опроса.

Тело представляет собой полный опрос, и отправленные вами варианты становятся вариантами опроса в указанном порядке. Каждый вариант сопоставляется по его `id`:

- Вариант, отправленный с `id` существующего варианта, сохраняет этот вариант и его голоса. Его метка и позиция обновляются в соответствии с отправленными данными.
- Вариант, отправленный без `id`, добавляется без голосов.
- Существующий вариант, который вы опускаете, удаляется вместе с отданными за него голосами. `totalVotes` уменьшается на то же количество.

Таким образом, чтобы добавить вариант, отправьте текущие варианты с их `id` и новый вариант без `id`. Чтобы удалить вариант, отправьте список без него. `id` вариантов находятся в опросе, возвращаемом запросом `GET /api/v1/polls/:commentId`.

Отправка без каких-либо `id` заменяет все варианты и удаляет все уже отданные голоса в опросе. Если в опросе есть голоса, это требует `replaceVotes=true`, а без него API отвечает `replace-votes-required`.

Другие поля также заменяются: если опустить `closesAt`, `privacy` или `requireVoteToSeeResults`, они сбрасываются к значениям по умолчанию. Чтобы изменить только одно поле, оставив остальные без изменений, используйте `PATCH /api/v1/polls/:commentId`.

[inline-code-attrs-start title = 'Пример cURL запроса Poll Put'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запроса Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** Required to keep none of the existing option ids when the poll has votes, since that deletes them all. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** The id of an existing option, to keep it and its votes. Omit to add a new option. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** The complete, ordered list. Existing options left out are removed with their votes. **/
    options: PollPutOption[]
    /** Must be in the future when the comment has no poll yet. Omit for a poll that stays open. **/
    closesAt?: string | null
    /** 0 anonymous (the default), 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа Poll Put'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Прочие замечания

- `id`, которого нет в опросе, или одинаковый `id`, указанный дважды, приводит к ошибке `poll-invalid`. У комментария без опроса ещё нет `id` вариантов, поэтому каждый отправляемый вариант должен опускать `id`.
- Приватность опроса можно сузить, но нельзя расширить после того, как в нём появились голоса.
- Этот API соблюдает настройки вашего сайта. Если опросы не включены для сайта или страницы, запрос завершится ошибкой `polls-disabled`.
- Заблокированный комментарий не может изменять свой опрос, и запрос завершится ошибкой `locked`.
- Подключённые виджеты обновляются в реальном времени, поэтому пользователи видят новый опрос без перезагрузки.