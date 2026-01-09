[api-resource-header-start name = 'QuestionResult'; route = 'PATCH /api/v1/question-results/:id'; creditsCost = 1; api-resource-header-end]

Цей маршрут дозволяє оновити один `QuestionResult`.

Наведена структура містить усі значення, які можна змінити:

[inline-code-attrs-start title = 'Структура QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchBody {
    urlId?: string
    anonUserId?: string
    userId?: string
    value?: string
    commentId?: string
    questionId?: string
    meta?: QuestionResultMeta[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Приклад cURL для оновлення QuestionResult'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-results/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"value": 5
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту для оновлення QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді на оновлення QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchResponse {
    status: 'success' | 'failed'
    /** Включено у разі невдачі. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** Включено у разі невдачі. **/
    reason?: string
}
[inline-code-end]

---