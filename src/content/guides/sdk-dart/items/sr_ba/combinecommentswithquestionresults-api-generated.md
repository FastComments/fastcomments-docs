## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| questionId | string | query | No |  |
| questionIds | array | query | No |  |
| urlId | string | query | No |  |
| startDate | string | query | No |  |
| forceRecalculate | boolean | query | No |  |
| minValue | number | query | No |  |
| maxValue | number | query | No |  |
| limit | number | query | No |  |

## Odgovor

Vraća: `CombineQuestionResultsWithCommentsResponse`

## Primjer

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfiguriši autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentari ispod da postaviš prefiks (npr. Bearer) za API ključ, po potrebi
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