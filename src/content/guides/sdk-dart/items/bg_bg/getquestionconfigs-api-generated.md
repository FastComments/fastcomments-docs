## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Отговор

Връща: `GetQuestionConfigsResponse`

## Пример

[inline-code-attrs-start title = 'Пример за getQuestionConfigs'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте оторизация на API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу за задаване на префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getQuestionConfigs(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionConfigs: $e\n');
}
[inline-code-end]