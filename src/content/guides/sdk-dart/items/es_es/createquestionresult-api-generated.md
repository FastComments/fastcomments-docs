## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |

## Respuesta

Devuelve: `CreateQuestionResultResponse`

## Ejemplo

[inline-code-attrs-start title = 'createQuestionResult Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar la autorización de la clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionResultBody = CreateQuestionResultBody(); // CreateQuestionResultBody | 

try {
    final result = api_instance.createQuestionResult(tenantId, createQuestionResultBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createQuestionResult: $e\n');
}
[inline-code-end]

---