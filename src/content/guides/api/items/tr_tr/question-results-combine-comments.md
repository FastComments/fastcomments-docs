[api-resource-header-start name = 'QuestionResultsCombinedWithComments'; route = 'GET /api/v1/question-results-aggregate/combine/comments'; creditsCost = 2; api-resource-header-end]

Bu, sonuçların yorumlarla birleştirildiği yerdir. Örneğin bir ürün için "son olumlu ve olumsuz yorumlar" grafiği oluşturmak için faydalıdır.

Değer aralığı (dahil), bir veya daha fazla soru ve bir başlangıç tarihi (dahil) ile arama yapabilirsiniz.

Yanıt yapısı şu şekildedir:

[inline-code-attrs-start title = 'QuestionResultsCombinedWithCommentsResponse Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Bu verinin hesaplandığı zamanı temsil eden bir tarih dizesi; önbellekten gelmiş olabilir. **/
    createdAt: string
    results: CommentAndResult[]
}
[inline-code-end]

Toplama için kullanılabilen sorgu parametreleri şunlardır:

[inline-code-attrs-start title = 'QuestionResultsCombineComments İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Bir veya daha fazla soru için sonuçları birleştirebilirsiniz. **/
    questionId: string | string[]
    startDate?: string | number
    urlId?: string
    minValue: number
    maxValue: number
    limit?: number
    forceRecalculate?: boolean
}
[inline-code-end]

İşte bir örnek istek:

[inline-code-attrs-start title = 'QuestionResultsCombineComments Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation/combine/comments?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id&minValue=5&maxValue=10'
[inline-code-end]

Örnek yanıt:

[inline-code-attrs-start title = 'QuestionResultsCombineComments Yanıt Örneği'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'QuestionResultsCombineComments Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsCombineCommentsResponse {
    status: 'success' | 'failed'
    /** Başarısızlık durumunda dahil edilir. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Başarısızlık durumunda dahil edilir. **/
    reason?: string
    data: QuestionResultsCombinedWithCommentsResponse
}
[inline-code-end]

### Önbellekleme ve Maliyet Notları

- `forceRecalculate` belirtildiğinde maliyet normal `2` yerine her zaman `10` olur.
- Önbellek süresi dolup veriler yeniden hesaplanırsa, `forceRecalculate` belirtilmemişse maliyet yine sabit `2` olur.
- Bu, önbelleğin kullanılmasını teşvik etmek içindir.

---