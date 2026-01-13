[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

Tukaj se izvaja združevanje rezultatov s komentarji. Uporabno na primer za ustvarjanje "nedavni pozitivni in negativni komentarji" grafikona za izdelek.

Iskanje lahko izvedete po razponu vrednosti (vključno), za eno ali več vprašanj in po začetnem datumu (vključno).

Struktura odgovora je naslednja:

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
    /** Niz z datumom, ki označuje, kdaj so bili ti podatki izračunani, saj lahko izhajajo iz predpomnilnika. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

Spodaj so parametri poizvedbe, ki so na voljo za agregacijo:

[inline-code-attrs-start title = 'Struktura zahtevka QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Rezultate lahko agregirate za eno ali več vprašanj. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

Tu je primer zahteve:

[inline-code-attrs-start title = 'Primer zahteve QuestionResultsCombineComments'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

Primer odgovora:

[inline-code-attrs-start title = 'Primer odgovora QuestionResultsCombineComments'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno ob napaki. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### Opombe o predpomnjenju in stroških

- Ko je določen `forceRecalculate`, je strošek vedno `10`, namesto običajnega `2`.
- Če predpomnilnik poteče in so podatki ponovno izračunani, je strošek še vedno konstanten `2`, če `forceRecalculate` ni določen.
- To je namenjeno spodbujanju uporabe predpomnilnika.

---