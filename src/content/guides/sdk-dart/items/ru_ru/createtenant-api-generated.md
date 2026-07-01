## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | query | Да |  |

## Ответ

Возвращает: `CreateTenantResponse`

## Пример

[inline-code-attrs-start title = 'createTenant Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Настройте авторизацию API-ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// раскомментируйте ниже, чтобы настроить префикс (например, Bearer) для API-ключа, если требуется
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantBody = CreateTenantBody(); // CreateTenantBody | 

try {
    final result = api_instance.createTenant(tenantId, createTenantBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenant: $e\n');
}
[inline-code-end]

---