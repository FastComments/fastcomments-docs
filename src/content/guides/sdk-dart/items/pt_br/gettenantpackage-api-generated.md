## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Returns: `GetTenantPackageResponse`

## Example

[inline-code-attrs-start title = 'Exemplo getTenantPackage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave de API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar prefixo (ex.: Bearer) para a chave de API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getTenantPackage(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getTenantPackage: $e\n');
}
[inline-code-end]