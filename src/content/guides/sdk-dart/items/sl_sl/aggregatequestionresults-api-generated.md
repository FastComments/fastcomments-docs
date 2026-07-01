## Parameters

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| questionId | string | query | Ne |  |
| questionIds | array | query | Ne |  |
| urlId | string | query | Ne |  |
| timeBucket | string | query | Ne |  |
| startDate | string | query | Ne |  |
| forceRecalculate | boolean | query | Ne |  |

## Odgovor

Vrne: `AggregateQuestionResultsResponse`

## Primer

[inline-code-attrs-start title = 'aggregateQuestionResults Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final questionId = questionId_example; // String | 
final questionIds = []; // List<String> | 
final urlId = urlId_example; // String | 
final timeBucket = ; // AggregateTimeBucket | 
final startDate = 2013-10-20T19:20:30+01:00; // DateTime | 
final forceRecalculate = true; // bool | 

try {
    final result = api_instance.aggregateQuestionResults(tenantId, AggregateQuestionResultsOptions(questionId: questionId, questionIds: questionIds, urlId: urlId, timeBucket: timeBucket, startDate: startDate, forceRecalculate: forceRecalculate));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->aggregateQuestionResults: $e\n');
}
[inline-code-end]