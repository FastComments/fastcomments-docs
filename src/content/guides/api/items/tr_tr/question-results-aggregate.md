[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Burası sonuçların toplandığı yerdir.

Agregasyon yanıt yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'QuestionResultsAggregationResult Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Geçerli veri noktası (tarih kovası veya sayfa) için değerden kaç tane olduğunu gösteren değer->adet haritası. **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Not - timeBucket belirtilmediğinde null olur. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Toplanan toplam sonuç sayısı. **/
    total: number
    /** Ortaya çıkan ağırlıklı ortalama. Ondalık sayıdır, genellikle iki ondalık veya daha az. **/
    average: number
    /** Bu verinin hesaplandığı zamanı gösteren tarih dizgisi; çünkü veriler önbellekten gelebilir. **/
    createdAt: string
}
[inline-code-end]

Here are the query parameters available for aggregation:

[inline-code-attrs-start title = 'QuestionResultsAggregation İstek Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** You can aggregate results for one or more questions. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Aggregate for a specific page. **/
    urlId?: string
    /** Aggregate for a specific user. **/
    userId?: string
    /** Force recalculate now and update the cache. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Here's an example request:

[inline-code-attrs-start title = 'QuestionResultsAggregation Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Example response:

[inline-code-attrs-start title = 'QuestionResultsAggregation Yanıt Örneği'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
    {
        "average": 8.33,
        "countsByValue": {
            "5": 1,
            "10": 2
        },
        "createdAt": "2023-08-30T00:00:00.000Z",
        "dataByUrlId": {
            "some-page": {
                "total": 3,
                "v": {
                    "5": 1,
                    "10": 2
                }
            }
        },
        "total": 3
    }
[inline-code-end]

[inline-code-attrs-start title = 'QuestionResultsAggregation Yanıt Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Performans Notları

- Önbellek bulunamadığında, agregasyonlar genellikle her milyon sonuç için beş saniye sürer.
- Aksi takdirde, istekler sabit zamanlıdır.

### Önbellekleme ve Maliyet Notları

- `forceRecalculate` belirtildiğinde maliyet her zaman `10` olur, normal `2` yerine.
- Önbellek süresi dolup veriler yeniden hesaplanırsa, `forceRecalculate` belirtilmemişse maliyet yine sabit `2`'dir. Önbellek, toplanan veri kümesinin boyutuna göre sona erer (30 saniye ile 5 dakika arasında değişebilir).
- Bu, önbelleğin kullanılmasını teşvik etmek içindir.

---