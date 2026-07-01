## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Odgovor

Vraća: `UpdateSubscriptionAPIResponse`

## Primer

[inline-code-attrs-start title = 'updateSubscription Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPIUserSubscriptionData = UpdateAPIUserSubscriptionData(); // UpdateAPIUserSubscriptionData | 
final userId = userId_example; // String | 

try {
    final result = api_instance.updateSubscription(tenantId, id, updateAPIUserSubscriptionData, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateSubscription: $e\n');
}
[inline-code-end]