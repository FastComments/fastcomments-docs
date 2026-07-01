## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | query | Sí |  |
| userId | string | query | Sí |  |
| id | string | path | Sí |  |

## Respuesta

Devuelve: `ChangeTicketStateResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo changeTicketState'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final id = id_example; // String | 
final changeTicketStateBody = ChangeTicketStateBody(); // ChangeTicketStateBody | 

try {
    final result = api_instance.changeTicketState(tenantId, userId, id, changeTicketStateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->changeTicketState: $e\n');
}
[inline-code-end]