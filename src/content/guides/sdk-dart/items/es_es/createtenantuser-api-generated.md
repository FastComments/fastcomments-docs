## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|--------|------|-----------|-------------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: `CreateTenantUserResponse`

## Ejemplo

[inline-code-attrs-start title = 'createTenantUser Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createTenantUserBody = CreateTenantUserBody(); // CreateTenantUserBody | 

try {
    final result = api_instance.createTenantUser(tenantId, createTenantUserBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createTenantUser: $e\n');
}
[inline-code-end]

---