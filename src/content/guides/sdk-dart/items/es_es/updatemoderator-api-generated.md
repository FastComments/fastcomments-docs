## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |

## Respuesta

Devuelve: `APIEmptyResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateModerator'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomenta lo siguiente para configurar el prefijo (p.ej. Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateModeratorBody = UpdateModeratorBody(); // UpdateModeratorBody | 

try {
    final result = api_instance.updateModerator(tenantId, id, updateModeratorBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateModerator: $e\n');
}
[inline-code-end]

---