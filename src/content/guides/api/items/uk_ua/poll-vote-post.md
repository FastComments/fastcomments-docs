[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Записує голос у опитуванні.

У виборця може бути не більше одного голосу в одному опитуванні. Виклик цього знову для того самого виборця переміщує його голос до нової опції замість додавання другого, а голосування за опцію, яку він вже обрав, нічого не робить.

Відповідь включає опитування, тому ви отримуєте оновлені підрахунки без другого запиту.

[inline-code-attrs-start title = 'Приклад cURL створення PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Приклад cURL створення анонімного PollVote'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура запиту створення PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** Один з userId або anonUserId є обов’язковим. **/
    userId?: string
    anonUserId?: string
    /** IP кінцевого користувача, використовується для анонімного обмеження швидкості. За замовчуванням — IP виклику. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді створення PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Включено у випадку помилки. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Включено у випадку помилки. **/
    reason?: string
    pollVote: PollVote
    /** Опитування з оновленими підрахунками. **/
    poll: CommentPoll
}
[inline-code-end]

### Анонімні голоси

Встановіть `anonUserId` замість `userId`, щоб записати голос для користувача, який не ввійшов у систему. Цей ідентифікатор не повинен відповідати жодному користувачу — він лише ідентифікує сесію, тому одна і та ж особа не рахується двічі.

Анонімне голосування має бути ввімкнене для вашого сайту. Якщо голосування обмежене лише зареєстрованими користувачами, голос з лише `anonUserId` завершиться помилкою `poll-login-required`.

Анонімні голоси також підлягають обмеженню швидкості за IP для кожного опитування, щоб запобігти заповненню опитування однією особою шляхом очищення їхньої сесії. Надішліть `ip` кінцевого користувача, щоб обмеження застосовувалося до нього, а не до вашого сервера.

### Інші примітки

- `userId` має бути користувачем, який існує на вашому сайті. Голоси для користувача, що належить іншому сайту, відхиляються.
- Голосування в закритому опитуванні завершується помилкою `poll-closed`.
- Цей API оновлює підрахунки в опитуванні та в реальному часі передає їх підключеним віджетам.

---