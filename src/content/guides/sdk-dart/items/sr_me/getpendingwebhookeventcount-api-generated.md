## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## Odgovor

Vraća: `GetPendingWebhookEventCountResponse`

## Primjer

[inline-code-attrs-start title = 'getPendingWebhookEventCount Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final externalId = externalId_example; // String | 
final eventType = eventType_example; // String | 
final type = type_example; // String | 
final domain = domain_example; // String | 
final attemptCountGT = 1.2; // double | 

try {
    final result = api_instance.getPendingWebhookEventCount(tenantId, GetPendingWebhookEventCountOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPendingWebhookEventCount: $e\n');
}
[inline-code-end]