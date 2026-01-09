[api-resource-header-start name = 'QuestionResult'; route = 'PATCH /api/v1/question-results/:id'; creditsCost = 1; api-resource-header-end]

Ta končna točka omogoča posodobitev enega `QuestionResult`.

Naslednja struktura predstavlja vse vrednosti, ki jih je mogoče spremeniti:

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

[inline-code-attrs-start title = 'Primer cURL za posodobitev QuestionResult'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/question-results/my-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"value": 5
}'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtevka za posodobitev QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za posodobitev QuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultPatchResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'unauthorized' | 'missing-api-key' | 'missing-id' | 'not-found' | 'empty-request' | 'invalid-input'
    /** Vključeno ob napaki. **/
    reason?: string
}
[inline-code-end]

---