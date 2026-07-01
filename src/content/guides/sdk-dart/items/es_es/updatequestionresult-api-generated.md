## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| id | string | path | Sí |  |

## Respuesta

Devuelve: `APIEmptyResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateQuestionResult'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p.ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateQuestionResultBody = UpdateQuestionResultBody(); // UpdateQuestionResultBody | 

try {
    final result = api_instance.updateQuestionResult(tenantId, id, updateQuestionResultBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateQuestionResult: $e\n');
}
[inline-code-end]