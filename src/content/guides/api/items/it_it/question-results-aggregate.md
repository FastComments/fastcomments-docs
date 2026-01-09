[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Qui avviene l'aggregazione dei risultati.

La struttura della risposta di aggregazione è la seguente:

[inline-code-attrs-start title = 'Struttura di QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Una mappa del valore al conteggio delle occorrenze di quel valore per il punto dati corrente (bucket di data o pagina). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Nota - è null quando timeBucket non è specificato. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Il numero totale di risultati aggregati. **/
    total: number
    /** La media ponderata risultante. È un float, di solito con due decimali o meno. **/
    average: number
    /** Una stringa di data che rappresenta quando questi dati sono stati calcolati, dato che potrebbero provenire dalla cache. **/
    createdAt: string
}
[inline-code-end]

Here are the query parameters available for aggregation:

[inline-code-attrs-start title = 'Struttura della richiesta QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Ecco una richiesta di esempio:

[inline-code-attrs-start title = 'Esempio di QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Esempio di risposta:

[inline-code-attrs-start title = 'Esempio di risposta di QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struttura della risposta di QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Note sulle prestazioni

- In caso di cache miss, le aggregazioni generalmente richiedono cinque secondi per milione di risultati.
- Altrimenti, le richieste sono a tempo costante.

### Note su caching e costi

- Quando `forceRecalculate` è specificato il costo è sempre `10`, invece del normale `2`.
- Se la cache scade e i dati vengono ricalcolati, il costo rimane comunque un valore costante di `2` se `forceRecalculate` non è specificato. La scadenza della cache dipende dalla dimensione del set di dati aggregato (può variare tra 30 secondi e 5 minuti).
- Questo serve a incentivare l'uso della cache.

---