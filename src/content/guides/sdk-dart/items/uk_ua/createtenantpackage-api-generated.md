## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes |  |

## Відповідь

Повертає: `CreateTenantPackageResponse`

## Приклад

[inline-code-attrs-start title = 'createTenantPackage Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Налаштуйте авторизацію API ключа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// розкоментуйте нижче, щоб налаштувати префікс (наприклад, Bearer) для API ключа, якщо потрібно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantPackageBody = CreateTenantPackageBody(); // CreateTenantPackageBody | 

try {
    final result = api_instance.createTenantPackage(tenantId, createTenantPackageBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenantPackage: $e\n');
}
[inline-code-end]

---