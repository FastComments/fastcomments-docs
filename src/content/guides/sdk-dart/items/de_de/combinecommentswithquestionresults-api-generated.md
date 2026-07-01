## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| questionId | string | query | Nein |  |
| questionIds | array | query | Nein |  |
| urlId | string | query | Nein |  |
| startDate | string | query | Nein |  |
| forceRecalculate | boolean | query | Nein |  |
| minValue | number | query | Nein |  |
| maxValue | number | query | Nein |  |
| limit | number | query | Nein |  |

## Antwort

Rückgabe: `CombineQuestionResultsWithCommentsResponse`

## Beispiel

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Entkommentieren Sie unten, um das Präfix (z. B. Bearer) für den API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final questionId = questionId_example; // String | 
final questionIds = []; // List<String> | 
final urlId = urlId_example; // String | 
final startDate = 2013-10-20T19:20:30+01:00; // DateTime | 
final forceRecalculate = true; // bool | 
final minValue = 1.2; // double | 
final maxValue = 1.2; // double | 
final limit = 1.2; // double | 

try {
    final result = api_instance.combineCommentsWithQuestionResults(tenantId, CombineCommentsWithQuestionResultsOptions(questionId: questionId, questionIds: questionIds, urlId: urlId, startDate: startDate, forceRecalculate: forceRecalculate, minValue: minValue, maxValue: maxValue, limit: limit));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->combineCommentsWithQuestionResults: $e\n');
}
[inline-code-end]