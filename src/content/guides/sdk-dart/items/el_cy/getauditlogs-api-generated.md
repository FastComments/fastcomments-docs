## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| limit | number | query | Όχι |  |
| skip | number | query | Όχι |  |
| order | string | query | Όχι |  |
| after | number | query | Όχι |  |
| before | number | query | Όχι |  |

## Απάντηση

Επιστρέφει: `GetAuditLogsResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'getAuditLogs Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε εξουσιοδότηση κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// αποσχολιάστε παρακάτω για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 
final order = ; // SORTDIR | 
final after = 1.2; // double | 
final before = 1.2; // double | 

try {
    final result = api_instance.getAuditLogs(tenantId, GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getAuditLogs: $e\n');
}
[inline-code-end]