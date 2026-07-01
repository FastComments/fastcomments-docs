## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: `CreateTenantPackageResponse`

## Example

[inline-code-attrs-start title = 'createTenantPackage Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Конфигуришите ауторизацију API кључа: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uklјučite ispod da postavite префикс (нпр. Bearer) за API кључ, ако је потребно
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