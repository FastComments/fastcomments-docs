[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Ovde se vrši agregacija rezultata.

Struktura odgovora za agregaciju je sledeća:

[inline-code-attrs-start title = 'Struktura QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Mapa vrednosti na broj pojavljivanja te vrednosti za trenutnu tačku podataka (vremenski bucket ili stranica). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Napomena - null je kada timeBucket nije naveden. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Ukupan broj agregiranih rezultata. **/
    total: number
    /** Dobijena ponderisana sredina. To je decimalni broj, obično sa dve decimale ili manje. **/
    average: number
    /** Datum u obliku stringa koji predstavlja kada su ovi podaci izračunati, pošto mogu poticati iz keša. **/
    createdAt: string
}
[inline-code-end]

Here are the query parameters available for aggregation:

[inline-code-attrs-start title = 'Struktura zahteva za QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Primer QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Example response:

[inline-code-attrs-start title = 'Primer odgovora za QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora za QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Napomene o performansama

- Ako dođe do promašaja keša, agregacije obično traju pet sekundi po milionu rezultata.
- U suprotnom, zahtevi se izvršavaju u konstantnom vremenu.

### Napomene o keširanju i troškovima

- Kada je `forceRecalculate` naveden, trošak je uvek `10`, umesto uobičajenih `2`.
- Ako keš istekne i podaci se ponovo izračunaju, trošak je i dalje konstantan `2` ako `forceRecalculate` nije naveden. Keš ističe u zavisnosti od veličine agregiranog skupa podataka (može varirati između 30 sekundi i 5 minuta).
- Ovo je da bi se podstaklo korišćenje keša.

---