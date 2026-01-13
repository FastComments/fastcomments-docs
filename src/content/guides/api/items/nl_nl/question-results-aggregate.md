---
[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Hier vindt de aggregatie van resultaten plaats.

De structuur van het aggregatieantwoord is als volgt:

[inline-code-attrs-start title = 'Structuur van QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Een map van waarde naar het aantal keer dat die waarde voorkomt voor het huidige datapunt (datumsegment of pagina). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Opmerking - is null wanneer timeBucket niet is opgegeven. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Het totale aantal geaggregeerde resultaten. **/
    total: number
    /** Het resulterende gewogen gemiddelde. Het is een float, gewoonlijk twee decimalen of minder. **/
    average: number
    /** Een datumstring die aangeeft wanneer deze gegevens zijn berekend, omdat ze uit de cache kunnen komen. **/
    createdAt: string
}
[inline-code-end]

Hier zijn de queryparameters die beschikbaar zijn voor aggregatie:

[inline-code-attrs-start title = 'Structuur van het QuestionResultsAggregation-verzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** U kunt resultaten aggregeren voor één of meer vragen. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Aggregeren voor een specifieke pagina. **/
    urlId?: string
    /** Aggregeren voor een specifieke gebruiker. **/
    userId?: string
    /** Forceer direct opnieuw berekenen en werk de cache bij. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Hier is een voorbeeldverzoek:

[inline-code-attrs-start title = 'Voorbeeld van QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Voorbeeldantwoord:

[inline-code-attrs-start title = 'Voorbeeldantwoord van QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structuur van het QuestionResultsAggregation-antwoord'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Opgenomen bij mislukking. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Prestatieopmerkingen

- Bij een cache-miss duren aggregaties doorgaans vijf seconden per miljoen resultaten.
- In andere gevallen zijn verzoeken constant qua tijd.

### Cache- en kostenopmerkingen

- Wanneer `forceRecalculate` is opgegeven, zijn de kosten altijd `10`, in plaats van de normale `2`.
- Als de cache verloopt en de gegevens opnieuw worden berekend, blijven de kosten een constante `2` als `forceRecalculate` niet is opgegeven. De cache verloopt afhankelijk van de grootte van de geaggregeerde dataset (kan variëren tussen 30 seconden en 5 minuten).
- Dit is om het gebruik van de cache te stimuleren.

---