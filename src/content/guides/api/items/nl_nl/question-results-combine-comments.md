[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

Dit is waar de combinatie van resultaten met reacties plaatsvindt. Handig voor het maken van een grafiek met "recente positieve en negatieve reacties" voor een product, bijvoorbeeld.

Je kunt zoeken via een bereik van waarden (inclusief), één of meer vragen, en op een begindatum (inclusief).

De response-structuur is als volgt:

[inline-code-attrs-start title = 'Structuur van QuestionResultsCombinedWithCommentsResponse'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SimpleComment {
    id: string
    commenterName: string
    userId?: string
    date: string
    commentHTML: string
}

interface SimpleQuestionResult {
    id: string
    value: number
}

interface CommentAndResult {
    comment: SimpleComment
    result: SimpleQuestionResult
}

interface QuestionResultsCombinedWithCommentsResponse {
    /** Een datumtekenreeks die aangeeft wanneer deze gegevens zijn berekend, aangezien ze uit de cache kunnen komen. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

Hier zijn de queryparameters die beschikbaar zijn voor aggregatie:

[inline-code-attrs-start title = 'Structuur van QuestionResultsCombineComments Request'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** You can aggregate results for one or more questions. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

Hier is een voorbeeldverzoek:

[inline-code-attrs-start title = 'Voorbeeld van QuestionResultsCombineComments'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

Voorbeeldresponse:

[inline-code-attrs-start title = 'Voorbeeldantwoord van QuestionResultsCombineComments'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "createdAt": "2023-08-30T00:00:00.000Z",
    "results": [
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-2",
                "commenterName": "Test",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 10
            }
        },
        {
            "comment": {
                "id": "some-id",
                "commentHTML": "test-0",
                "commenterName": "Some Name",
                "date": "2023-08-30T00:00:00.000Z",
                "userId": "some-user-id"
            },
            "result": {
                "id": "some-id",
                "value": 5
            }
        }
    ]
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de response van QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij fout. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### Caching- en kostenopmerkingen

- Wanneer `forceRecalculate` is opgegeven, zijn de kosten altijd `10`, in plaats van de normale `2`.
- Als de cache verloopt en de gegevens opnieuw worden berekend, blijven de kosten constant `2` wanneer `forceRecalculate` niet is opgegeven.
- Dit is bedoeld om het gebruik van de cache te stimuleren.

---