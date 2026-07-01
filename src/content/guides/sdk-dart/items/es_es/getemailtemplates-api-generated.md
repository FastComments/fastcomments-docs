## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|------------|-----------|--------------|
| tenantId | string | query | Sí |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: `GetEmailTemplatesResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo getEmailTemplates'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomentar abajo para configurar el prefijo (p.ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getEmailTemplates(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getEmailTemplates: $e\n');
}
[inline-code-end]