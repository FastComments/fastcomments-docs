## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| questionId | string | query | Non |  |
| questionIds | array | query | Non |  |
| urlId | string | query | Non |  |
| startDate | string | query | Non |  |
| forceRecalculate | boolean | query | Non |  |
| minValue | number | query | Non |  |
| maxValue | number | query | Non |  |
| limit | number | query | Non |  |

## Réponse

Retourne : `CombineQuestionResultsWithCommentsResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple combineCommentsWithQuestionResults'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de clé API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// décommentez ci-dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
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