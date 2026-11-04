[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Sonuçların toplandığı (agregasyon) yer burasıdır.

Agregasyon yanıt yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'QuestionResultsAggregationResult Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** A map of value to count of occurrences of that value for the current data point (date bucket or page). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Note - is null when timeBucket not specified. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** The total number of results aggregated. **/
    total: number
    /** The resulting weighted average. It is a float, usually two decimals or fewer. **/
    average: number
    /** A date string representing when this data was calculated, since it might come from cache. **/
    createdAt: string
}
[inline-code-end]

Agregasyon için kullanılabilen sorgu parametreleri şunlardır:

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

Örnek bir istek:

[inline-code-attrs-start title = 'QuestionResultsAggregation Örneği'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Örnek yanıt:

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

- Önbellek isabeti olmadığında agregasyonlar genellikle milyon sonuç başına beş saniye sürer.
- Aksi takdirde, istekler sabit zamanda tamamlanır.

### Önbellekleme ve Maliyet Notları

- `forceRecalculate` belirtildiğinde maliyet normal `2` yerine her zaman `10` olur.
- Önbellek süresi dolarsa ve veriler yeniden hesaplanırsa, `forceRecalculate` belirtilmediği takdirde maliyet yine sabit `2` olur. Önbellek, agregasyon yapılan veri kümesinin boyutuna göre sona erer (30 saniye ile 5 dakika arasında değişebilir).
- Bu, önbelleğin kullanılmasını teşvik etmek içindir.