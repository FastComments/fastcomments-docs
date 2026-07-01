## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Отговор

Връща: `CreateQuestionResultResponse`

## Пример

[inline-code-attrs-start title = 'Пример за createQuestionResult'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте упълномощаване с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу за настройка на префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionResultBody = CreateQuestionResultBody(); // CreateQuestionResultBody | 

try {
    final result = api_instance.createQuestionResult(tenantId, createQuestionResultBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createQuestionResult: $e\n');
}
[inline-code-end]