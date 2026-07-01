---
## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|--------|------|-----------|-------------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Respuesta

Devuelve: `APIEmptyResponse`

## Ejemplo

[inline-code-attrs-start title = 'updateTenant Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p.ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

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

---