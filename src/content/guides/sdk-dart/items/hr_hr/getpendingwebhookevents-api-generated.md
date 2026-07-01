## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| commentId | string | query | Ne |  |
| externalId | string | query | Ne |  |
| eventType | string | query | Ne |  |
| type | string | query | Ne |  |
| domain | string | query | Ne |  |
| attemptCountGT | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Returns: `GetPendingWebhookEventsResponse`

## Primjer

[inline-code-attrs-start title = 'getPendingWebhookEvents Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte dolje za postavljanje prefiksa (npr. Bearer) za API ključ, ako je potrebno
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