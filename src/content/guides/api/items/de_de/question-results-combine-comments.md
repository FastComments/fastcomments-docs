[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

Hier findet die Kombination von Ergebnissen mit Kommentaren statt. Nützlich zum Beispiel für die Erstellung eines "letzte positive und negative Kommentare"-Diagramms für ein Produkt.

Sie können über einen Wertebereich (inklusive), eine oder mehrere Fragen und nach einem Startdatum (inklusive) suchen.

Die Antwortstruktur ist wie folgt:

[inline-code-attrs-start title = 'QuestionResultsCombinedWithCommentsResponse Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** A date string representing when this data was calculated, since it might come from cache. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

Hier sind die für die Aggregation verfügbaren Abfrageparameter:

[inline-code-attrs-start title = 'QuestionResultsCombineComments Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Hier ist ein Beispiel für eine Anfrage:

[inline-code-attrs-start title = 'QuestionResultsCombineComments Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

Beispielantwort:

[inline-code-attrs-start title = 'QuestionResultsCombineComments Antwort Beispiel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'QuestionResultsCombineComments Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### Caching- und Kostenhinweise

- Wenn `forceRecalculate` angegeben wird, betragen die Kosten immer `10` anstelle der normalen `2`.
- Wenn der Cache abläuft und Daten neu berechnet werden, betragen die Kosten immer noch konstant `2`, wenn `forceRecalculate` nicht angegeben ist.
- Dies soll die Nutzung des Caches fördern.
