[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Tukaj poteka agregacija rezultatov.

Struktura odziva agregacije je naslednja:

[inline-code-attrs-start title = 'Struktura QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Mapa vrednosti v število pojavitev te vrednosti za trenutno podatkovno točko (časovni segment ali stran). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Opomba - je null, ko timeBucket ni naveden. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Skupno število agregiranih rezultatov. **/
    total: number
    /** Izračunan tehtani povpreček. Gre za plavajoče število (float), običajno z največ dvema decimalnima mestoma. **/
    average: number
    /** Niz datuma, ki predstavlja, kdaj so bili ti podatki izračunani, saj so lahko iz predpomnilnika. **/
    createdAt: string
}
[inline-code-end]

Tu so razpoložljivi parametri poizvedbe za agregacijo:

[inline-code-attrs-start title = 'Struktura zahteve QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Lahko agregirate rezultate za eno ali več vprašanj. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Agregiraj za določeno stran. **/
    urlId?: string
    /** Agregiraj za določenega uporabnika. **/
    userId?: string
    /** Prisili ponovno izračunanje zdaj in posodobi predpomnilnik. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Primer zahteve:

[inline-code-attrs-start title = 'Primer zahteve QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Primer odziva:

[inline-code-attrs-start title = 'Primer odziva QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odziva QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** Vključeno v primeru napake. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Vključeno v primeru napake. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Opombe o zmogljivosti

- Pri neujemajočem predpomnilniku (cache miss) agregacije običajno trajajo pet sekund na milijon rezultatov.
- V nasprotnem primeru so zahtevki v konstantnem času.

### Opombe o predpomnjenju in stroških

- Ko je `forceRecalculate` določen, je strošek vedno `10`, namesto običajnih `2`.
- Če predpomnilnik poteče in se podatki ponovno izračunajo, je strošek še vedno konstanten `2`, če `forceRecalculate` ni naveden. Predpomnilnik poteče glede na velikost agregiranega nabora podatkov (lahko se giblje med 30 sekundami in 5 minutami).
- To spodbuja uporabo predpomnilnika.

---