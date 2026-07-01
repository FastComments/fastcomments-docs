## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Ответ

Возвращает: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'updateTenantPackage Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы установить префикс (например, Bearer) для API-ключа, если необходимо
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantPackageBody = UpdateTenantPackageBody(); // UpdateTenantPackageBody | 

try {
    final result = api_instance.updateTenantPackage(tenantId, id, updateTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenantPackage: $e\n');
}
[inline-code-end]

---