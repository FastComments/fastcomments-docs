[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Εδώ γίνεται η συγκέντρωση των αποτελεσμάτων.

Η δομή απάντησης συγκέντρωσης είναι η ακόλουθη:

[inline-code-attrs-start title = 'Δομή QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Εδώ είναι οι διαθέσιμες παράμετροι ερωτήματος για συγκέντρωση:

[inline-code-attrs-start title = 'Δομή Αιτήματος QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Εδώ είναι ένα παράδειγμα αιτήματος:

[inline-code-attrs-start title = 'Παράδειγμα QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Παράδειγμα απάντησης:

[inline-code-attrs-start title = 'Παράδειγμα Απάντησης QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Δομή Απάντησης QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Σημειώσεις Απόδοσης

- Για αστοχία προσωρινής μνήμης, οι συγκεντρώσεις γενικά χρειάζονται πέντε δευτερόλεπτα ανά εκατομμύριο αποτελέσματα.
- Διαφορετικά, τα αιτήματα είναι σταθερού χρόνου.

### Σημειώσεις Προσωρινής Αποθήκευσης και Κόστους

- Όταν καθορίζεται το `forceRecalculate`, το κόστος είναι πάντα `10`, αντί του κανονικού `2`.
- Αν η προσωρινή μνήμη λήξει και τα δεδομένα επανυπολογιστούν, το κόστος παραμένει σταθερό `2` αν δεν καθοριστεί το `forceRecalculate`. Η προσωρινή μνήμη λήγει με βάση το μέγεθος του συνόλου δεδομένων που συγκεντρώνεται (μπορεί να κυμαίνεται μεταξύ 30 δευτερολέπτων και 5 λεπτών).
- Αυτό γίνεται για να ενθαρρύνει τη χρήση της προσωρινής μνήμης.
