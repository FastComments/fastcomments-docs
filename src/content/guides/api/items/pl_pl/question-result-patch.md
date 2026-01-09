[api-resource-header-start name = 'QuestionResult'; route = 'PATCH /api/v1/question-results/:id'; creditsCost = 1; api-resource-header-end]

Ta trasa umożliwia aktualizację pojedynczego `QuestionResult`.

Poniższa struktura przedstawia wszystkie wartości, które można zmienić:

[inline-code-attrs-start title = 'Struktura QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Przykład aktualizacji QuestionResult (cURL)'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-results/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"value": 5
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania aktualizacji QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi aktualizacji QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]