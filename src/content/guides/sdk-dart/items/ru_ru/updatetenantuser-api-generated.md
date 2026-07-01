## Parameters

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | string | query | No |  |

## Response

Возвращает: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'Пример updateTenantUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantUserBody = UpdateTenantUserBody(); // UpdateTenantUserBody | 
final updateComments = updateComments_example; // String | 

try {
    final result = api_instance.updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenantUser: $e\n');
}
[inline-code-end]