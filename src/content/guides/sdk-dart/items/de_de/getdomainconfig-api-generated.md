## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| domain | string | path | Yes |  |

## Rückgabe

Rückgabe: `GetDomainConfigResponse`

## Beispiel

[inline-code-attrs-start title = 'getDomainConfig Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurieren Sie die API-Schlüsselautorisation: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// auskommentieren Sie unten, um das Präfix (z. B. Bearer) für den API‑Schlüssel einzurichten, falls nötig
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final domain = domain_example; // String | 

try {
    final result = api_instance.getDomainConfig(tenantId, domain);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getDomainConfig: $e\n');
}
[inline-code-end]