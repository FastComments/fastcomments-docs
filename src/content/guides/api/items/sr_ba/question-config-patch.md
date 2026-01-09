[api-resource-header-start name = 'QuestionConfig'; route = 'PATCH /api/v1/question-configs/:id'; creditsCost = 1; api-resource-header-end]

Ова рута омогућава ажурирање појединачног `QuestionConfig`.

Следећа структура представља све вредности које се могу изменити:

[inline-code-attrs-start title = 'Структура QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchBody {
    name?: string
    question?: string
    helpText?: string
    /** Пазите! Промјена type може пореметити извјештавање ако су min/max различити. **/
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

[inline-code-attrs-start title = 'cURL примјер за ажурирање QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-configs/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "some new question text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Структура захтјева за ажурирање QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура одговора за ажурирање QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспјеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** Укључено у случају неуспјеха. **/
    reason?: string
}
[inline-code-end]

---