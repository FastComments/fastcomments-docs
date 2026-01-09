---
[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Ovde se vrši agregacija rezultata.

Struktura odgovora agregacije je sledeća:

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
    /** Dobijeni ponderisani prosek. To je decimalni broj (float), obično sa dve decimale ili manje. **/
    average: number
    /** Datum u obliku stringa koji predstavlja kada su ovi podaci izračunati, jer mogu doći iz keša. **/
    createdAt: string
}
[inline-code-end]

Evo parametara upita dostupnih za agregaciju:

[inline-code-attrs-start title = 'Struktura zahteva QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Možete agregirati rezultate za jedno ili više pitanja. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Agregiraj za određenu stranicu. **/
    urlId?: string
    /** Agregiraj za određenog korisnika. **/
    userId?: string
    /** Forsiraj ponovno izračunavanje odmah i ažuriraj keš. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Evo primera zahteva:

[inline-code-attrs-start title = 'Primer QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Primer odgovora:

[inline-code-attrs-start title = 'Primer odgovora QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno u slučaju greške. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Napomene o performansama

- Kod promašaja keša (cache miss) agregacije obično traju oko pet sekundi po milionu rezultata.
- U suprotnom, zahtevi su konstantnog vremena.

### Napomene o keširanju i troškovima

- Kada je navedeno `forceRecalculate`, trošak je uvek `10`, umesto uobičajenih `2`.
- Ako keš istekne i podaci se ponovo izračunaju, trošak je i dalje konstantan `2` ako `forceRecalculate` nije naveden. Keš ističe na osnovu veličine agregiranog skupa podataka (može varirati između 30 sekundi i 5 minuta).
- Ovo služi kao podsticaj za korišćenje keša.

---