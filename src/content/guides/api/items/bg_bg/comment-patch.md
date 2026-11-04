[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Този API крайна точка предоставя възможност за обновяване на един коментар.

Забележки:

- Този API може да обнови уиджета за коментари "на живо", ако е желано (това увеличава базовия `creditsCost` от `1` на `2`).
  - Това може да направи миграцията на коментари между страници "на живо" (промяна на `urlId`).
  - Миграциите струват допълнителни `2` кредита, тъй като страниците се изчисляват предварително и това натоварва процесора.
- За разлика от API за създаване, този API НЯМА автоматично да създава потребителски обекти в нашата система, ако е предоставен имейл.
- Коментарите, обновени чрез този API, все пак могат да бъдат проверявани за спам, ако е желано.
- Конфигурация като максимална дължина на коментара, ако е настроена чрез администраторската страница Customization Rule, ще се прилага и тук.
- За да позволите на потребителите да обновяват текста на своя коментар, можете просто да посочите `comment` в тялото на заявката. Ние ще генерираме получения `commentHTML`.
  - Ако дефинирате и `comment`, и `commentHTML`, няма да генерираме автоматично HTML-а.
  - Ако потребителят добави упоменания или хаштагове в новия си текст, те все пак ще бъдат обработени като при `POST` API-я.
- При обновяване на `commenterEmail` на коментар е най-добре да посочите и `userId`. В противен случай трябва да сте сигурни, че потребителят с този имейл принадлежи към вашия tenant, или заявката ще се провали.
- Ако целевият коментар е заключен (`isLocked: true`), заявката се отхвърля с `code: 'locked'`. Първо отключете коментара, обновете го, а след това го заключете отново, ако е желано.


[inline-code-attrs-start title = 'Минимален пример за PATCH заявка на коментар с cURL'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на PATCH заявка за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Потребителят, извършващ обновяването. При желание може да се използва за проверка дали може да редактира коментара.  **/
    contextUserId?: string
	/** Да ли да проверим дали новият коментар изглежда като спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Дали коментарът да се появи "на живо" за потребителите, които разглеждат инстанции на уиджета за коментари със същия urlId. ЗАБЕЛЕЖКА: Удвоява цената в кредити от 1 на 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на PATCH отговор за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Включва се при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Включва се при неуспех. **/
    reason?: string
}
[inline-code-end]