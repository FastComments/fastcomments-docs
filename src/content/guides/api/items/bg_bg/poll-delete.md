[api-resource-header-start name = 'Poll'; route = 'DELETE /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Премахва анкета от нейния коментар, заедно с всички гласове, дадени за нея. Самият коментар остава непроменен.

Изтриването на коментара също премахва неговата анкета и гласовете, затова това е необходимо само когато искате да запазите коментара.

[inline-code-attrs-start title = 'Пример за cURL заявка за изтриване на анкета'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура на заявка за изтриване на анкета'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура на отговор за изтриване на анкета'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteResponse {
    status: 'success' | 'failed'
    /** Включено при неуспех. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Включено при неуспех. **/
    reason?: string
}
[inline-code-end]

---