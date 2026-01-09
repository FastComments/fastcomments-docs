[api-resource-header-start name = 'QuestionResultsAggregate'; route = 'GET /api/v1/question-results-aggregate'; creditsCost = 2; api-resource-header-end]

Tutaj odbywa się agregacja wyników.

Struktura odpowiedzi agregacji wygląda następująco:

[inline-code-attrs-start title = 'Struktura QuestionResultsAggregationResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultDataPoint {
    /** Mapa wartości na liczbę wystąpień tej wartości dla bieżącego punktu danych (kubełka daty lub strony). **/
    v: Map<ValueAsString, number>
    total: number
}

interface QuestionResultsAggregationResult {
    /** Uwaga - jest null, gdy timeBucket nie jest określony. **/
    dataByDateBucket?: Map<DateString, QuestionResultDataPoint>
    dataByUrlId?: Map<URLIdString, QuestionResultDataPoint>
    countsByValue?: Map<ValueAsString, number>
    /** Całkowita liczba zagregowanych wyników. **/
    total: number
    /** Obliczona średnia ważona. Jest to liczba zmiennoprzecinkowa, zazwyczaj z dwoma miejscami po przecinku lub mniej. **/
    average: number
    /** Łańcuch daty reprezentujący moment obliczenia tych danych, ponieważ mogą pochodzić z pamięci podręcznej. **/
    createdAt: string
}
[inline-code-end]

Oto parametry zapytania dostępne dla agregacji:

[inline-code-attrs-start title = 'Struktura żądania QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregateRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Możesz agregować wyniki dla jednego lub więcej pytań. **/
    questionId: string | string[]
    startDate?: string | number
    timeBucket?: 'day' | 'month' | 'year'
    /** Agreguj dla konkretnej strony. **/
    urlId?: string
    /** Agreguj dla konkretnego użytkownika. **/
    userId?: string
    /** Wymuś ponowne obliczenie teraz i zaktualizuj pamięć podręczną. **/
    forceRecalculate?: boolean
}
[inline-code-end]

Oto przykład żądania:

[inline-code-attrs-start title = 'Przykład QuestionResultsAggregation'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/question-results-aggregation?tenantId=demo&API_KEY=DEMO_API_SECRET&questionId=some-question-id'
[inline-code-end]

Przykładowa odpowiedź:

[inline-code-attrs-start title = 'Przykład odpowiedzi QuestionResultsAggregation'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odpowiedzi QuestionResultsAggregation'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface QuestionResultsAggregationResponse {
    status: 'success' | 'failed'
    /** Dołączane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Dołączane w przypadku niepowodzenia. **/
    reason?: string
    data: QuestionResultsAggregationResult
}
[inline-code-end]

### Uwagi dotyczące wydajności

- W przypadku braku w pamięci podręcznej agregacje zazwyczaj zajmują pięć sekund na milion wyników.
- W przeciwnym razie żądania wykonują się w stałym czasie.

### Uwagi dotyczące pamięci podręcznej i kosztów

- Gdy `forceRecalculate` jest określone, koszt zawsze wynosi `10`, zamiast standardowego `2`.
- Jeżeli pamięć podręczna wygaśnie i dane zostaną przeliczone, koszt wciąż wynosi stałe `2`, jeśli `forceRecalculate` nie jest określone. Pamięć podręczna wygasa w zależności od rozmiaru agregowanego zestawu danych (może się wahać między 30 sekundami a 5 minutami).
- Ma to na celu zachęcenie do korzystania z pamięci podręcznej.