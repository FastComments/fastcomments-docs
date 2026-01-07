[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Her sker aggregering af resultater.

Aggregerings-svarstrukturen er som følger:

[inline-code-attrs-start title = 'QuestionResultsAggregationResult Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Her er query-parametrene tilgængelige for aggregering:

[inline-code-attrs-start title = 'QuestionResultsAggregation Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Her er et eksempel på en anmodning:

[inline-code-attrs-start title = 'QuestionResultsAggregation Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Eksempel på svar:

[inline-code-attrs-start title = 'QuestionResultsAggregation Svar Eksempel'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'QuestionResultsAggregation Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Ydelsesbemærkninger

- For en cache-miss tager aggregeringer generelt fem sekunder pr. million resultater.
- Ellers er anmodninger konstant-tid.

### Caching og Omkostningsbemærkninger

- Når `forceRecalculate` er angivet, er omkostningen altid `10` i stedet for de normale `2`.
- Hvis cachen udløber og data genberegnes, er omkostningen stadig en konstant `2`, hvis `forceRecalculate` ikke er angivet. Cachen udløber baseret på datasættets størrelse, der aggregeres (kan variere mellem 30 sekunder og 5 minutter).
- Dette er for at tilskynde brug af cachen.
