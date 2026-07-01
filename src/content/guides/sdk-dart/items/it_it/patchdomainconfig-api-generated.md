## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Risposta

Restituisce: `PatchDomainConfigResponse`

## Esempio

[inline-code-attrs-start title = 'Esempio patchDomainConfig'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurare l'autorizzazione della chiave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// rimuovere il commento qui sotto per impostare il prefisso (ad es. Bearer) per la chiave API, se necessario
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domainToUpdate = domainToUpdate_example; // String | 
final patchDomainConfigParams = PatchDomainConfigParams(); // PatchDomainConfigParams | 

try {
    final result = api_instance.patchDomainConfig(tenantId, domainToUpdate, patchDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->patchDomainConfig: $e\n');
}
[inline-code-end]

---