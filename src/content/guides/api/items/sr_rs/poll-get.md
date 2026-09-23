[api-resource-header-start name = 'Poll'; route = 'GET /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

Чита анкету придружену коментару, са тренутним бројем гласова.

Анкете се такође враћају уз сам коментар преко API‑ја за коментаре, па користите ово када желите само резултате, а не цео коментар.

[inline-code-attrs-start title = 'Пример cURL захтева за добијање анкете'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за добијање анкете'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за добијање анкете'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollGetResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** Укључено у случају неуспеха. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

Коментар без анкете, обрисани коментар и ID коментара који не постоји сви одговарају на исти начин, са `poll-not-found`.