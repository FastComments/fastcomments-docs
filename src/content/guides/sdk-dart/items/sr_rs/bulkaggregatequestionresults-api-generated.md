## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| forceRecalculate | boolean | query | Не |  |

## Одговор

Враћа: `BulkAggregateQuestionResultsResponse`

## Пример

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Подесите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// уклони коментар испод да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final bulkAggregateQuestionResultsRequest = BulkAggregateQuestionResultsRequest(); // BulkAggregateQuestionResultsRequest | 
final forceRecalculate = true; // bool | 

try {
    final result = api_instance.bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->bulkAggregateQuestionResults: $e\n');
}
[inline-code-end]