## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|-----------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Отговор

Връща: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'Пример за deleteQuestionConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте упълномощаване с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// премахнете коментара по-долу, за да настроите префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteQuestionConfig(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteQuestionConfig: $e\n');
}
[inline-code-end]

---