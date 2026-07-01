## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Risposta

Restituisce: `APIEmptyResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio updateQuestionConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configura l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// decommenta la riga seguente per impostare il prefisso (ad es. Bearer) per la chiave API, se necessario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateQuestionConfigBody = UpdateQuestionConfigBody(); // UpdateQuestionConfigBody | 

try {
    final result = api_instance.updateQuestionConfig(tenantId, id, updateQuestionConfigBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateQuestionConfig: $e\n');
}
[inline-code-end]