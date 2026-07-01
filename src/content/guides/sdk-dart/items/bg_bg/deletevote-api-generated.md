## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| editKey | string | query | Не |  |

## Отговор

Връща: `VoteDeleteResponse`

## Пример

[inline-code-attrs-start title = 'deleteVote Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте авторизация за API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Разкоментирайте по-долу, за да настроите префикс (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final editKey = editKey_example; // String | 

try {
    final result = api_instance.deleteVote(tenantId, id, editKey);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteVote: $e\n');
}
[inline-code-end]