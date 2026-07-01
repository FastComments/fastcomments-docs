## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | consulta | Sí |  |
| id | string | ruta | Sí |  |

## Respuesta

Devuelve: `APIGetUserBadgeResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getUserBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p.ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUserBadge(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadge: $e\n');
}
[inline-code-end]