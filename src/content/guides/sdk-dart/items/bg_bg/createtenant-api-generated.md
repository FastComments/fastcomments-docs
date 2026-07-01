## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Отговор

Връща: `CreateTenantResponse`

## Пример

[inline-code-attrs-start title = 'Пример за createTenant'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигурирайте упълномощаване с API ключ: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// разкоментирайте по-долу за настройка на префикс (например Bearer) за API ключ, ако е необходимо
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