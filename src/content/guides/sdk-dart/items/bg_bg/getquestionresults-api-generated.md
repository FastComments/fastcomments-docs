## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| startDate | string | query | No |  |
| questionId | string | query | No |  |
| questionIds | string | query | No |  |
| skip | number | query | No |  |

## Response

Returns: `GetQuestionResultsResponse`

## Example

[inline-code-attrs-start title = 'getQuestionResults Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте упълномощаване с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да настроите префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final userId = userId_example; // String | 
final startDate = startDate_example; // String | 
final questionId = questionId_example; // String | 
final questionIds = questionIds_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getQuestionResults(tenantId, GetQuestionResultsOptions(urlId: urlId, userId: userId, startDate: startDate, questionId: questionId, questionIds: questionIds, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionResults: $e\n');
}
[inline-code-end]

---