[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за актуализиране на единичен коментар.

Забележки:

- Този API може да актуализира уиджета за коментари "на живо" ако желаете (това увеличава базовата `creditsCost` от `1` на `2`).
  - Това може да направи мигрирането на коментари между страници "на живо" (промяна на `urlId`).
  - Миграциите струват допълнително `2` кредита, тъй като страниците се предварително изчисляват и това е CPU интензивно.
- За разлика от API за създаване, този API НЯМА автоматично да създава потребителски обекти в нашата система, ако е предоставен имейл.
- Коментарите, актуализирани чрез този API, все още могат да бъдат проверявани за спам, ако желаете.
- Конфигурацията като максимална дължина на коментара, ако е конфигурирана чрез административната страница за правила за персонализация, ще се прилага тук.
- За да позволите на потребителите да актуализират текста на коментара си, можете просто да посочите `comment` в тялото на заявката. Ние ще генерираме резултатния `commentHTML`.
  - Ако дефинирате и `comment`, и `commentHTML`, ние няма автоматично да генерираме HTML.
  - Ако потребителят добави споменавания или хаштагове в новия си текст, той все още ще бъде обработен като `POST` API.
- Когато актуализирате `commenterEmail` на коментар, е най-добре също да посочите `userId`. В противен случай трябва да се уверите, че потребителят с този имейл принадлежи на вашия tenant, или заявката ще се провали.


[inline-code-attrs-start title = 'Пример за минимален PATCH на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявката за PATCH на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** The user doing the update. If desired can be used to check that they can edit the comment.  **/
    contextUserId?: string
	/** Should we check if the new comment looks like spam?  **/
    doSpamCheck?: 'true' | 'false'
	/** Whether the comment should appear "live" to users viewing instances of the comment widget with the same urlId. NOTE: Doubles credit cost from 1 to 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за PATCH на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
