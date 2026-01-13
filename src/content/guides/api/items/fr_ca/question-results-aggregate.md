[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

C'est ici que l'agrégation des résultats se produit.

La structure de réponse d'agrégation est la suivante:

[inline-code-attrs-start title = 'Structure de QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Voici les paramètres de requête disponibles pour l'agrégation:

[inline-code-attrs-start title = 'Structure de Requête de QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

Voici un exemple de requête:

[inline-code-attrs-start title = 'Exemple de QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Exemple de réponse:

[inline-code-attrs-start title = 'Exemple de Réponse de QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Structure de Réponse de QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### Notes de Performance

- Pour un échec de cache, les agrégations prennent généralement cinq secondes par million de résultats.
- Sinon, les requêtes sont en temps constant.

### Notes sur le Cache et les Coûts

- Lorsque `forceRecalculate` est spécifié, le coût est toujours `10`, au lieu du `2` normal.
- Si le cache expire et que les données sont recalculées, le coût reste un constant `2` si `forceRecalculate` n'est pas spécifié. Le cache expire en fonction de la taille de l'ensemble de données agrégé (peut varier entre 30 secondes et 5 minutes).
- Ceci est pour encourager l'utilisation du cache.
