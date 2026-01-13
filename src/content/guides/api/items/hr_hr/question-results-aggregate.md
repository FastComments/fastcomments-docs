[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Ovdje se vrši agregacija rezultata.

Struktura odgovora agregacije je sljedeća:

[inline-code-attrs-start title = 'Struktura QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Mapa vrijednosti na broj pojavljivanja te vrijednosti za trenutnu točku podataka (grupa po datumu ili stranica). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Napomena - null je kada timeBucket nije specificiran. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Ukupan broj rezultata koji su agregirani. **/
    total: number
    /** Dobiveni ponderirani prosjek. To je float, obično s dvije decimale ili manje. **/
    average: number
    /** Datum u obliku stringa koji predstavlja kada su ovi podaci izračunati, budući da mogu doći iz cachea. **/
    createdAt: string
}
[inline-code-end]

Ovdje su dostupni parametri upita za agregaciju:

[inline-code-attrs-start title = 'Struktura zahtjeva QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Prisilno ponovno izračunaj sada i ažuriraj predmemoriju. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Evo primjera zahtjeva:

[inline-code-attrs-start title = 'Primjer QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Primjer odgovora:

[inline-code-attrs-start title = 'Primjer odgovora QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno pri neuspjehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Uključeno pri neuspjehu. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Napomene o performansama

- Ako nema u cacheu (cache miss), agregacije obično traju pet sekundi po milijunu rezultata.
- U suprotnom, zahtjevi se izvršavaju u konstantnom vremenu.

### Napomene o keširanju i troškovima

- Kada je `forceRecalculate` naveden, trošak je uvijek `10`, umjesto uobičajenih `2`.
- Ako cache istekne i podaci se ponovno izračunaju, trošak je i dalje konstantan `2` ako `forceRecalculate` nije naveden. Cache istječe ovisno o veličini prikupljenog skupa podataka (može varirati između 30 sekundi i 5 minuta).
- Ovo služi kao poticaj za korištenje cachea.

---