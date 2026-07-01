## Параметри

| Назва | Type | Location | Обов'язково | Опис |
|------|------|----------|------------|------|
| tenantId | string | query | Так |  |
| urlId | string | query | Ні |  |
| userId | string | query | Ні |  |
| startDate | string | query | Ні |  |
| questionId | string | query | Ні |  |
| questionIds | string | query | Ні |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: `GetQuestionResultsResponse`

## Приклад

[inline-code-attrs-start title = 'getQuestionResults Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштуйте авторизацію API‑ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API‑ключа, якщо потрібно
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