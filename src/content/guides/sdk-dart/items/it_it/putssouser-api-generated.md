## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |
| updateComments | boolean | query | No |  |

## Risposta

Restituisce: `PutSSOUserAPIResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio putSSOUser'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configura l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// decommenta sotto per configurare il prefisso (es. Bearer) per la chiave API, se necessario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateAPISSOUserData = UpdateAPISSOUserData(); // UpdateAPISSOUserData | 
final updateComments = true; // bool | 

try {
    final result = api_instance.putSSOUser(tenantId, id, updateAPISSOUserData, updateComments);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->putSSOUser: $e\n');
}
[inline-code-end]