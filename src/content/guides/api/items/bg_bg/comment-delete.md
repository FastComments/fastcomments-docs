[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Тази API крайна точка предоставя възможност за изтриване на коментар.

Бележки:

- Този API може да актуализира коментарния widget "live" ако е желано (това увеличава `creditsCost` от `1` до `2`).
- Този API ще изтрие всички дочерни коментари.
- Ако целевият коментар е заключен (`isLocked: true`), заявката се отхвърля с `code: 'locked'`. Първо отключете коментара, след което го изтрийте.

[inline-code-attrs-start title = 'Пример на cURL заявка за изтриване на коментар'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Структура на DELETE заявка за коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** Потребителят, извършващ актуализацията. При желание може да се използва за проверка дали може да изтрие коментара.  **/
    contextUserId?: string
	/** Дали коментарът трябва да бъде изтрит "на живо" за потребителите, които гледат екземпляри на коментарния widget със същия urlId. NOTE: Удва разхода на кредити от 1 на 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за DELETE на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** Включено при неуспех. **/
    reason?: string
}
[inline-code-end]

---