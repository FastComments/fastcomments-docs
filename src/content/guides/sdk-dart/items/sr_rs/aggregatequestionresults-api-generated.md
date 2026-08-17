## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| questionId | string | query | Не |  |
| questionIds | array | query | Не |  |
| urlId | string | query | Не |  |
| timeBucket | string | query | Не |  |
| startDate | string | query | Не |  |
| forceRecalculate | boolean | query | Не |  |

## Response

Враћа: `AggregateQuestionResultsResponse`

## Example

[inline-code-attrs-start title = 'Пример aggregateQuestionResults'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуришите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// одкоментаришите испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
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