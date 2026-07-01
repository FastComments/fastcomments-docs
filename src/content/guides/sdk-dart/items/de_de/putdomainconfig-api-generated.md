## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|---------------|
| tenantId | string | query | Yes |  |
| domainToUpdate | string | path | Yes |  |

## Antwort

Returns: `PutDomainConfigResponse`

## Beispiel

[inline-code-attrs-start title = 'putDomainConfig Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API‑Schlüssel‑Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// Kommentieren Sie die unten stehende Zeile aus, um ein Präfix (z. B. Bearer) für den API‑Schlüssel einzurichten, falls erforderlich

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domainToUpdate = domainToUpdate_example; // String | 
final updateDomainConfigParams = UpdateDomainConfigParams(); // UpdateDomainConfigParams | 

try {
    final result = api_instance.putDomainConfig(tenantId, domainToUpdate, updateDomainConfigParams);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->putDomainConfig: $e\n');
}
[inline-code-end]