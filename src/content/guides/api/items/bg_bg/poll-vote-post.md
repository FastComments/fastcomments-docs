[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

Записва глас за анкета.

Гласоподавателят може да има най-много един глас за анкета. Повторно извикване за същия гласоподавател премества гласа му към новата
опция, вместо да добавя втори, а гласуването за опцията, която вече е избрал, не прави нищо.

Отговорът включва анкетата, така че получавате актуализираните броячи без второ заявяване.

[inline-code-attrs-start title = 'Пример за създаване на PollVote с cURL'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Пример за създаване на анонимен PollVote с cURL'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Структура на заявка за създаване на PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** Едно от userId или anonUserId е задължително. **/
    userId?: string
    anonUserId?: string
    /** IP адресът на крайния потребител, използван за анонимното ограничение по скорост. По подразбиране е IP адресът на извикващия. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за създаване на PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Включено при неуспех. **/
    reason?: string
    pollVote: PollVote
    /** Анкетата с актуализираните й броячи. **/
    poll: CommentPoll
}
[inline-code-end]

### Анонимни гласове

Задайте `anonUserId` вместо `userId`, за да запишете глас за някой, който не е влязъл. Това ID не трябва да
съответства на потребител никъде – то просто идентифицира сесията, така че същият човек да не бъде преброен два пъти.

Анонимното гласуване трябва да бъде активирано за вашия сайт. Ако гласуването е ограничено до влезли потребители, глас с единствено
`anonUserId` се проваля с `poll-login-required`.

Анонимните гласове също са ограничени по скорост за всеки IP за всяка анкета, за да се предотврати един човек да запълни анкета, като изчисти
своята сесия. Изпратете `ip` на крайния потребител, за да се прилага ограничението към него, а не към вашия сървър.

### Други бележки

- `userId` трябва да бъде потребител, който съществува във вашия сайт. Гласовете за потребител, принадлежащ на друг сайт, се отхвърлят.
- Гласуването в затворена анкета се проваля с `poll-closed`.
- Този API актуализира брояча в анкетата и ги изпраща в живо към свързаните уиджети.

---