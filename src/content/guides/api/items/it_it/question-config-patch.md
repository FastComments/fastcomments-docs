[api-resource-header-start name = 'QuestionConfig'; route = 'PATCH /api/v1/question-configs/:id'; creditsCost = 1; api-resource-header-end]

Questa route permette di aggiornare un singolo `QuestionConfig`.

La seguente struttura rappresenta tutti i valori che possono essere modificati:

[inline-code-attrs-start title = 'Struttura di QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchBody {
    name?: string
    question?: string
    helpText?: string
    /** Attenzione! Cambiare il tipo potrebbe compromettere i report se min/max sono diversi. **/
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

[inline-code-attrs-start title = 'Esempio cURL di aggiornamento QuestionConfig'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-configs/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "some new question text"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della richiesta di aggiornamento QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struttura della risposta di aggiornamento QuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionConfigPatchResponse {
    status: 'success' | 'failed'
    /** Incluso in caso di errore. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** Incluso in caso di errore. **/
    reason?: string
}
[inline-code-end]