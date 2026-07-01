## Параметри

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Одговор

Vraća: `GetTenantPackagesResponse`

## Пример

[inline-code-attrs-start title = 'Primer getTenantPackages'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Подесите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// одкоментаришите доле да подесите префикс (нпр. Bearer) за API кључ, ако је потребно
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getTenantPackages(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantPackages: $e\n');
}
[inline-code-end]