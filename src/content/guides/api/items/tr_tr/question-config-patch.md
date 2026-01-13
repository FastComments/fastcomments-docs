[api-resource-header-start name = 'QuestionConfig'; route = 'PATCH /api/v1/question-configs/:id'; creditsCost = 1; api-resource-header-end]

Bu rota tek bir `QuestionConfig`'i güncelleme olanağı sağlar.

Aşağıdaki yapı değiştirilebilecek tüm değerleri temsil eder:

[inline-code-attrs-start title = 'QuestionConfig Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchBody {
    name?: string
    question?: string
    helpText?: string
    /** Dikkat! type değiştirildiğinde raporlama min/max farklıysa etkilenebilir. **/
    type?: QuestionConfigType
    numStars?: number
    min?: number
    max?: number
    defaultValue?: number
    labelNegative?: string
    labelPositive?: string
    subQuestionIds?: string[]
    alwaysShowSubQuestions?: boolean
    reportingOrder?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Güncelleme cURL Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-configs/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "some new question text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Güncelleme İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'QuestionConfig Güncelleme Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchResponse {
    status: 'success' | 'failed'
    /** Hata durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** Hata durumunda dahil edilir. **/
    reason?: string
}
[inline-code-end]

---