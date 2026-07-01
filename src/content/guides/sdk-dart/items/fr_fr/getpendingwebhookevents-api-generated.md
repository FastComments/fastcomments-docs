## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |
| skip | number | query | No |  |

## Réponse

Retourne : `GetPendingWebhookEventsResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getPendingWebhookEvents'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurer l'autorisation de la clé API : api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final externalId = externalId_example; // String | 
final eventType = eventType_example; // String | 
final type = type_example; // String | 
final domain = domain_example; // String | 
final attemptCountGT = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getPendingWebhookEvents(tenantId, GetPendingWebhookEventsOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPendingWebhookEvents: $e\n');
}
[inline-code-end]

---