## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|--------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| contextUserId | string | query | No |  |
| isLive | boolean | query | No |  |

## Respuesta

Devuelve: `DeleteCommentResult`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de deleteComment'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Descomente a continuación para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final contextUserId = contextUserId_example; // String | 
final isLive = true; // bool | 

try {
    final result = api_instance.deleteComment(tenantId, id, DeleteCommentOptions(contextUserId: contextUserId, isLive: isLive));
    print(result);
} catch (e) {
    print('Excepción al llamar a DefaultApi->deleteComment: $e\n');
}
[inline-code-end]