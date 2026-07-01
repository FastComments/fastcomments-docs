## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| email | string | path | Yes |  |

## Respuesta

Returns: `GetSSOUserByEmailAPIResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getSSOUserByEmail'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p.ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final email = email_example; // String | 

try {
    final result = api_instance.getSSOUserByEmail(tenantId, email);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getSSOUserByEmail: $e\n');
}
[inline-code-end]