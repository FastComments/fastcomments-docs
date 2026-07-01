## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Respuesta

Devuelve: `CreateEmailTemplateResponse`

## Ejemplo

[inline-code-attrs-start title = 'createEmailTemplate Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abajo para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createEmailTemplateBody = CreateEmailTemplateBody(); // CreateEmailTemplateBody | 

try {
    final result = api_instance.createEmailTemplate(tenantId, createEmailTemplateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createEmailTemplate: $e\n');
}
[inline-code-end]