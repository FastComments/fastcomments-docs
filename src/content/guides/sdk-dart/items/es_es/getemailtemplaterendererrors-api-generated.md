## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|--------|------|------------|-------------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: `GetEmailTemplateRenderErrorsResponse`

## Ejemplo

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar la autorización de la clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomentar abajo para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getEmailTemplateRenderErrors(tenantId, id, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getEmailTemplateRenderErrors: $e\n');
}
[inline-code-end]