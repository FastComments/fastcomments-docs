## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| id | string | path | Ja |  |
| userId | string | query | Nee |  |

## Respons

Retourneert: `APIEmptyResponse`

## Voorbeeld

[inline-code-attrs-start title = 'updateNotification Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureren van API-sleutelautorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment hieronder om prefix (bijv. Bearer) voor API-sleutel in te stellen, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateNotificationBody = UpdateNotificationBody(); // UpdateNotificationBody | 
final userId = userId_example; // String | 

try {
    final result = api_instance.updateNotification(tenantId, id, updateNotificationBody, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateNotification: $e\n');
}
[inline-code-end]