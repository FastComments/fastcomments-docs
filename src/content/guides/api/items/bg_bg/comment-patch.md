[api-resource-header-start name = 'Comment'; route = 'PATCH /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

Тази крайна точка на API предоставя възможност за актуализиране на един коментар.

Notes:

- This API can update the comment widget "live" if desired (this increases base `creditsCost` from `1` to `2`).
  - This can make migrating comments between pages "live" (changing `urlId`).
  - Migrations cost an additional `2` credits as pages are precalculated and this is CPU intensive.
- Unlike the create API, this API will NOT automatically create user objects in our system if email is provided.
- Comments updated via this API can still be checked for spam if desired.
- Configuration such as max comment length, if configured via the Customization Rule admin page, will apply here.
- To allow users to update their comment text, you can just specify `comment` in the request body. We will generate the resulting `commentHTML`.
  - If you define both `comment` and `commentHTML` we will not automatically generate the HTML.
  - If the user adds mentions or hashtags in their new text, it will still be processed like the `POST` API.
- When updating `commenterEmail` on a comment, it is best to also specify `userId`. Otherwise, you must ensure the user with this email belongs to your tenant, or the request will fail.  
- If the target comment is locked (`isLocked: true`), the request is rejected with `code: 'locked'`. Unlock the comment first, update it, then re-lock if desired.


[inline-code-attrs-start title = 'Минимален пример за PATCH на коментар (cURL)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET&isLive=true' \
  --header 'Content-Type: application/json' \
  --data '{
	"comment": "some-new-comment-text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за PATCH на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPatchQueryParams {
    tenantId: string
    API_KEY: string
	/** Потребителят, който извършва актуализацията. При желание може да се използва, за да се провери дали може да редактира коментара.  **/
    contextUserId?: string
	/** Трябва ли да проверим дали новият коментар изглежда като спам?  **/
    doSpamCheck?: 'true' | 'false'
	/** Дали коментарът трябва да се появи "в реално време" за потребителите, разглеждащи инстанции на коментарния widget със същия urlId. ЗАБЕЛЕЖКА: Удвоява цената в кредити от 1 до 2. **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговора за PATCH на коментар'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentPatchResponse {
    status: 'success' | 'failed'
    /** Включва се при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-comment' | 'comment-too-big' | 'hash-tags-readonly' | 'mentions-readonly' | 'invalid-user' | 'unauthorized' | 'invalid-date' | 'invalid-name' | 'invalid-name-is-email' | 'banned' | 'invalid-email' | 'invalid-input' | 'missing-id' | 'not-found' | 'locked'
    /** Включва се при неуспех. **/
    reason?: string
}
[inline-code-end]