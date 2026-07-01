## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|-----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nej |  |
| questionIds | array | query | Nej |  |
| urlId | string | query | Nej |  |
| timeBucket | string | query | Nej |  |
| startDate | string | query | Nej |  |
| forceRecalculate | boolean | query | Nej |  |

## Svar

Returnerer: `AggregateQuestionResultsResponse`

## Eksempel

[inline-code-attrs-start title = 'aggregateQuestionResults Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøgleautorisation: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Uncomment nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøgle, hvis nødvendigt
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