## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | query | Sí |  |
| domain | string | path | Sí |  |

## Respuesta

Devuelve: `DeleteDomainConfigResponse`

## Ejemplo

[inline-code-attrs-start title = 'deleteDomainConfig Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de la clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domain = domain_example; // String | 

try {
    final result = api_instance.deleteDomainConfig(tenantId, domain);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteDomainConfig: $e\n');
}
[inline-code-end]