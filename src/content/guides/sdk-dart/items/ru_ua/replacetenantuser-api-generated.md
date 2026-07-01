## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| updateComments | string | query | No |  |

## Ответ

Возвращает: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'replaceTenantUser Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию ключа API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для ключа API, если нужно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final replaceTenantUserBody = ReplaceTenantUserBody(); // ReplaceTenantUserBody | 
final updateComments = updateComments_example; // String | 

try {
    final result = api_instance.replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->replaceTenantUser: $e\n');
}
[inline-code-end]