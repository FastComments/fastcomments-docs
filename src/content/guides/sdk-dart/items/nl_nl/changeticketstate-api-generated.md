## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |
| id | string | path | Yes |  |

## Respons

Retourneert: `ChangeTicketStateResponse`

## Voorbeeld

[inline-code-attrs-start title = 'changeTicketState Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configureer API-sleutel autorisatie: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// ontcomment hieronder om prefix in te stellen (bijv. Bearer) voor API-sleutel, indien nodig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final id = id_example; // String | 
final changeTicketStateBody = ChangeTicketStateBody(); // ChangeTicketStateBody | 

try {
    final result = api_instance.changeTicketState(tenantId, userId, id, changeTicketStateBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->changeTicketState: $e\n');
}
[inline-code-end]

---