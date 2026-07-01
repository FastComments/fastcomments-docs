## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Отговор

Връща: `APIEmptySuccessResponse`

## Пример

[inline-code-attrs-start title = 'updateUserBadge Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте удостоверяване с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Премахнете коментара по-долу, за да настроите префикс (например Bearer) за API ключа, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateUserBadgeParams = UpdateUserBadgeParams(); // UpdateUserBadgeParams | 

try {
    final result = api_instance.updateUserBadge(tenantId, id, updateUserBadgeParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateUserBadge: $e\n');
}
[inline-code-end]