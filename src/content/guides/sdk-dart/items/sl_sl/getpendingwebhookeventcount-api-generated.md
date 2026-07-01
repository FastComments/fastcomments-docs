## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | poizvedba | Da |  |
| commentId | string | poizvedba | Ne |  |
| externalId | string | poizvedba | Ne |  |
| eventType | string | poizvedba | Ne |  |
| type | string | poizvedba | Ne |  |
| domain | string | poizvedba | Ne |  |
| attemptCountGT | number | poizvedba | Ne |  |

## Odgovor

Vrne: `GetPendingWebhookEventCountResponse`

## Primer

[inline-code-attrs-start title = 'Primer getPendingWebhookEventCount'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte avtorizacijo API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte spodaj za nastavitev predpone (npr. Bearer) za API ključ, po potrebi
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