## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Επιστρέφει: `APIEmptyResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα deletePendingWebhookEvent'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ξεσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
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