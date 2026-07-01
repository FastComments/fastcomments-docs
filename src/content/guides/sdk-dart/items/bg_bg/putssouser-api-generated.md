## Параметри

| Име | Тип | Location | Required | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| updateComments | boolean | query | Не |  |

## Отговор

Returns: `PutSSOUserAPIResponse`

## Пример

[inline-code-attrs-start title = 'putSSOUser Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте авторизация с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментарирайте по-долу, за да настроите представка (например Bearer) за API ключ, ако е необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPISSOUserData = UpdateAPISSOUserData(); // UpdateAPISSOUserData | 
final updateComments = true; // bool | 

try {
    final result = api_instance.putSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->putSSOUser: $e\n');
}
[inline-code-end]