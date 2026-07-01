## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| questionId | string | query | Hayır |  |
| questionIds | array | query | Hayır |  |
| urlId | string | query | Hayır |  |
| startDate | string | query | Hayır |  |
| forceRecalculate | boolean | query | Hayır |  |
| minValue | number | query | Hayır |  |
| maxValue | number | query | Hayır |  |
| limit | number | query | Hayır |  |

## Yanıt

Döndürür: `CombineQuestionResultsWithCommentsResponse`

## Örnek

[inline-code-attrs-start title = 'combineCommentsWithQuestionResults Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API anahtarı yetkilendirmesini yapılandır: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Aşağıdakini yorumdan çıkararak API anahtarı için önek (ör. Bearer) ayarlayın, gerekirse
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