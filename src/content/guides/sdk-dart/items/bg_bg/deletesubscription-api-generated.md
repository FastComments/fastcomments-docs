## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Не |  |

## Отговор

Връща: `DeleteSubscriptionAPIResponse`

## Пример

[inline-code-attrs-start title = 'Пример за deleteSubscription'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте оторизация с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу, за да зададете префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.deleteSubscription(tenantId, id, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteSubscription: $e\n');
}
[inline-code-end]

---