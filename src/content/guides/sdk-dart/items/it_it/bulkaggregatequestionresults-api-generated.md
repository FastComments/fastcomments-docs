## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| forceRecalculate | boolean | query | No |  |

## Risposta

Restituisce: `BulkAggregateQuestionResultsResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio bulkAggregateQuestionResults'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configura l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// decommenta sotto per impostare il prefisso (es. Bearer) per la chiave API, se necessario
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