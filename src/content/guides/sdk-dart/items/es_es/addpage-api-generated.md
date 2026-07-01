## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: `AddPageAPIResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo addPage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomentar a continuación para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createAPIPageData = CreateAPIPageData(); // CreateAPIPageData | 

try {
    final result = api_instance.addPage(tenantId, createAPIPageData);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addPage: $e\n');
}
[inline-code-end]