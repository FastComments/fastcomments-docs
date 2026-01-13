[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

Ovdje se kombiniraju rezultati s komentarima. Korisno za izradu grafikona "nedavni pozitivni i negativni komentari" za proizvod, na primjer.

Možete pretraživati prema rasponu vrijednosti (uključivo), jednom ili više pitanja i prema datumu početka (uključivo).

Struktura odgovora je sljedeća:

[inline-code-attrs-start title = 'Struktura QuestionResultsCombinedWithCommentsResponse'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Datum u obliku stringa koji predstavlja kada su ovi podaci izračunati, budući da mogu biti iz predmemorije. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

Ovo su parametri upita dostupni za agregaciju:

[inline-code-attrs-start title = 'Struktura zahtjeva QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Možete agregirati rezultate za jedno ili više pitanja. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

Evo primjera zahtjeva:

[inline-code-attrs-start title = 'Primjer zahtjeva QuestionResultsCombineComments'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

Primjer odgovora:

[inline-code-attrs-start title = 'Primjer odgovora QuestionResultsCombineComments'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### Napomene o predmemoriranju i troškovima

- Kada je naveden `forceRecalculate`, trošak je uvijek `10`, umjesto uobičajenih `2`.
- Ako predmemorija istekne i podaci se ponovno izračunaju, trošak je i dalje konstantan `2` ako `forceRecalculate` nije naveden.
- Ovo potiče korištenje predmemorije.

---