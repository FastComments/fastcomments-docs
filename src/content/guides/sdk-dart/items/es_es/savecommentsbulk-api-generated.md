## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## Respuesta

Devuelve: `SaveCommentsBulkResponse`

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo saveCommentsBulk'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createCommentParams = [List<CreateCommentParams>()]; // List<CreateCommentParams> | 
final isLive = true; // bool | 
final doSpamCheck = true; // bool | 
final sendEmails = true; // bool | 
final populateNotifications = true; // bool | 

try {
    final result = api_instance.saveCommentsBulk(tenantId, createCommentParams, SaveCommentsBulkOptions(isLive: isLive, doSpamCheck: doSpamCheck, sendEmails: sendEmails, populateNotifications: populateNotifications));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->saveCommentsBulk: $e\n');
}
[inline-code-end]