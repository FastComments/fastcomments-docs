## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|------------|----------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| updateComments | boolean | query | Нет |  |

## Ответ

Returns: `PatchSSOUserAPIResponse`

## Пример

[inline-code-attrs-start title = 'Пример patchSSOUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настроить авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментировать ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPISSOUserData = UpdateAPISSOUserData(); // UpdateAPISSOUserData | 
final updateComments = true; // bool | 

try {
    final result = api_instance.patchSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchSSOUser: $e\n');
}
[inline-code-end]