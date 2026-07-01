## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Отговор

Връща: `APIEmptyResponse`

## Пример

[inline-code-attrs-start title = 'Пример за updateTenant'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте удостоверяване с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// откоментирайте по-долу, за да настроите префикс (например Bearer) за API ключ, ако е необходимо

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateTenantBody = UpdateTenantBody(); // UpdateTenantBody | 

try {
    final result = api_instance.updateTenant(tenantId, id, updateTenantBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateTenant: $e\n');
}
[inline-code-end]