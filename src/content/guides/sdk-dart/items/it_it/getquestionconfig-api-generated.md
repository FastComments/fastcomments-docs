## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |

## Risposta

Ritorna: `GetQuestionConfigResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio getQuestionConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurare l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// decommentare qui sotto per impostare il prefisso (ad esempio Bearer) per la chiave API, se necessario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getQuestionConfig(tenantId, id);
    print(result);
} catch (e) {
    print('Eccezione durante la chiamata a DefaultApi->getQuestionConfig: $e\n');
}
[inline-code-end]