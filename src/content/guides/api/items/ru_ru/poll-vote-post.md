[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Записывает голос в опрос.

У избирателя может быть не более одного голоса в опросе. Повторный вызов для того же избирателя перемещает его голос на новый вариант вместо добавления второго, а голосование за уже выбранный вариант не приводит к изменению.

Ответ включает опрос, поэтому вы получаете обновлённые подсчёты без дополнительного запроса.

[inline-code-attrs-start title = 'Пример cURL создания PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Пример cURL создания анонимного PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запроса создания PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура ответа создания PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Анонимные голоса

Установите `anonUserId` вместо `userId`, чтобы записать голос для пользователя, который не вошёл в систему. Этот идентификатор не обязан соответствовать какому‑либо пользователю — он просто идентифицирует сессию, поэтому один и тот же человек не будет засчитан дважды.

Анонимное голосование должно быть включено для вашего сайта. Если голосование ограничено только авторизованными пользователями, голос с единственным `anonUserId` завершится ошибкой `poll-login-required`.

Анонимные голоса также ограничены по скорости на каждый IP в каждом опросе, чтобы предотвратить заполнение опроса одним человеком путём очистки своей сессии. Передайте `ip` конечного пользователя, чтобы ограничение применялось к нему, а не к вашему серверу.

### Прочие замечания

- `userId` должен соответствовать пользователю, существующему на вашем сайте. Голоса за пользователя, принадлежащего другому сайту, отклоняются.
- Голосование в закрытом опросе завершается ошибкой `poll-closed`.
- Этот API обновляет подсчёты в опросе и в реальном времени передаёт их подключённым виджетам.

---