## Parámetros

| Nombre | Tipo | Ubicación | Obligatorio | Descripción |
|--------|------|-----------|-------------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Respuesta

Devuelve: `APIGetUserBadgeProgressListResponse`

## Ejemplo

[inline-code-attrs-start title = 'getUserBadgeProgressList Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorización de clave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente a continuación para configurar el prefijo (p. ej., Bearer) para la clave API, si es necesario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getUserBadgeProgressList(tenantId, GetUserBadgeProgressListOptions(userId: userId, limit: limit, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressList: $e\n');
}
[inline-code-end]