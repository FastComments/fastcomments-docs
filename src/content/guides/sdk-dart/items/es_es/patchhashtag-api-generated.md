## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|------|------|----------|-------------|-------------|
| tenantId | string | query | Sí |  |
| tag | string | path | Sí |  |

## Respuesta

Devuelve: `UpdateHashTagResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo patchHashTag'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configura la autorización de la clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomenta a continuación para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final updateHashTagBody = UpdateHashTagBody(); // UpdateHashTagBody | 

try {
    final result = api_instance.patchHashTag(tenantId, tag, updateHashTagBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchHashTag: $e\n');
}
[inline-code-end]