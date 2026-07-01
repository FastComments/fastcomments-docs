## Παράμετροι

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| forceRecalculate | boolean | query | Όχι |  |

## Απάντηση

Επιστρέφει: `BulkAggregateQuestionResultsResponse`

## Παράδειγμα

[inline-code-attrs-start title = 'bulkAggregateQuestionResults Παράδειγμα'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Διαμορφώστε την εξουσιοδότηση του κλειδιού API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ξεσχολιάστε παρακάτω για να ρυθμίσετε το πρόθεμα (π.χ. Bearer) για το κλειδί API, εάν χρειάζεται
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final bulkAggregateQuestionResultsRequest = BulkAggregateQuestionResultsRequest(); // BulkAggregateQuestionResultsRequest | 
final forceRecalculate = true; // bool | 

try {
    final result = api_instance.bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest, forceRecalculate);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->bulkAggregateQuestionResults: $e\n');
}
[inline-code-end]