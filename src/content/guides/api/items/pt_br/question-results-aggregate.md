[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

É aqui que a agregação dos resultados acontece.

A estrutura de resposta da agregação é a seguinte:

[inline-code-attrs-start title = 'Estrutura de QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Um mapa de valor para o número de ocorrências desse valor para o ponto de dados atual (intervalo de data ou página). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Observação - é nulo quando timeBucket não for especificado. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** O número total de resultados agregados. **/
    total: number
    /** A média ponderada resultante. É um float, geralmente com duas casas decimais ou menos. **/
    average: number
    /** Uma string de data que representa quando esses dados foram calculados, pois podem vir do cache. **/
    createdAt: string
}
[inline-code-end]

Aqui estão os parâmetros de consulta disponíveis para agregação:

[inline-code-attrs-start title = 'Estrutura de Requisição de QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Você pode agregar resultados de uma ou mais perguntas. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Agregar para uma página específica. **/
    urlId?: string
    /** Agregar para um usuário específico. **/
    userId?: string
    /** Forçar recálculo agora e atualizar o cache. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Aqui está um exemplo de requisição:

[inline-code-attrs-start title = 'Exemplo de QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Exemplo de resposta:

[inline-code-attrs-start title = 'Exemplo de Resposta de QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Estrutura de Resposta de QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** Incluído em caso de falha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Incluído em caso de falha. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Observações de Desempenho

- No caso de uma falha de cache, as agregações geralmente levam cinco segundos por milhão de resultados.
- Caso contrário, as requisições têm tempo constante.

### Observações sobre Cache e Custo

- Quando `forceRecalculate` é especificado, o custo é sempre `10`, em vez do normal `2`.
- Se o cache expirar e os dados forem recalculados, o custo ainda será um constante `2` se `forceRecalculate` não estiver especificado. O cache expira com base no tamanho do conjunto de dados agregado (pode variar entre 30 segundos e 5 minutos).
- Isto é para incentivar o uso do cache.

---