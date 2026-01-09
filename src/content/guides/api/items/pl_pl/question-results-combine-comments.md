[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

Tutaj odbywa się łączenie wyników z komentarzami. Przydatne np. do tworzenia wykresu „ostatnie pozytywne i negatywne komentarze” dla produktu.

Możesz wyszukiwać w zakresie wartości (włącznie), dla jednego lub kilku pytań oraz według daty początkowej (włącznie).

Struktura odpowiedzi jest następująca:

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
    /** Ciąg daty wskazujący, kiedy dane zostały obliczone — może pochodzić z pamięci podręcznej. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

Oto parametry zapytania dostępne do agregacji:

[inline-code-attrs-start title = 'Struktura żądania QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Możesz agregować wyniki dla jednego lub większej liczby pytań. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

Oto przykład żądania:

[inline-code-attrs-start title = 'Przykład żądania QuestionResultsCombineComments'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

Przykładowa odpowiedź:

[inline-code-attrs-start title = 'Przykład odpowiedzi QuestionResultsCombineComments'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odpowiedzi QuestionResultsCombineComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### Pamięć podręczna i uwagi dotyczące kosztów

- Gdy podano `forceRecalculate`, koszt zawsze wynosi `10`, zamiast normalnego `2`.
- Jeśli pamięć podręczna wygaśnie i dane zostaną przeliczone, koszt nadal wynosi `2`, jeśli nie podano `forceRecalculate`.
- Ma to na celu zachęcenie do korzystania z pamięci podręcznej.

---