## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: `APIEmptyResponse`

## Primer

[inline-code-attrs-start title = 'deletePendingWebhookEvent Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO: Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentarišite donje da podesite prefiks (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deletePendingWebhookEvent(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deletePendingWebhookEvent: $e\n');
}
[inline-code-end]