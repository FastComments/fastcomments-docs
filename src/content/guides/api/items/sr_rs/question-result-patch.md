[api-resource-header-start name = 'QuestionResult'; route = 'PATCH /api/v1/question-results/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава ажурирање једног `QuestionResult`.

Следећа структура представља све вредности које се могу променити:

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

[inline-code-attrs-start title = 'Пример cURL захтева за ажурирање QuestionResult'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-results/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"value": 5
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтева за ажурирање QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за ажурирање QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** Укључено у случају неуспеха. **/
    reason?: string
}
[inline-code-end]

---