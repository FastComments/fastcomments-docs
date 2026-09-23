[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Редактирует опрос, не затрагивая его голоса. Используйте это для исправления опечатки в вопросе или варианте ответа, для закрытия или повторного открытия опроса, либо для изменения того, кто может видеть, кто проголосовал.

Опции идентифицируются по их `id`, а `PATCH` переименовывает указанные вами опции. Чтобы добавить, удалить или переупорядочить опции, отправьте полный список опций в `PUT /api/v1/polls/:commentId`: опции, отправленные вместе с их id, сохраняют свои голоса.

Каждое поле является необязательным, но должно быть указано хотя бы одно.

[inline-code-attrs-start title = 'Пример cURL для обновления опроса'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Пример cURL для закрытия опроса сейчас'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса обновления опроса'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа обновления опроса'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### Other Notes

- Указание id опции, которой нет в опросе, приводит к ошибке `poll-invalid`, а не к тихому игнорированию.  
- Метки должны оставаться уникальными в пределах опроса, учитывая опции, которые вы не меняете.  
- В отличие от создания опроса, здесь `closesAt` может быть в прошлом — так вы закрываете опрос немедленно.  
- Приватность опроса можно сузить, но нельзя расширить после того, как в нём появились голоса.  
- Заблокированный комментарий не может изменить свой опрос, и запрос завершится ошибкой `locked`.