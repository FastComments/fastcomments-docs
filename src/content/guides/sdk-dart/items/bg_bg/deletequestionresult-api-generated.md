## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|---------------|----------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |

## Отговор

Връща: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'deleteQuestionResult Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте оторизация за API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteQuestionResult(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteQuestionResult: $e\n');
}
[inline-code-end]

---