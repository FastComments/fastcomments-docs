## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|--------------|---------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Ja |  |

## Antwort

Rückgabe: `GetPageByURLIdAPIResponse`

## Beispiel

[inline-code-attrs-start title = 'getPageByURLId Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO API-Schlüssel-Authentifizierung konfigurieren: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// unten auskommentieren, um Präfix (z.B. Bearer) für API-Schlüssel einzurichten, falls erforderlich
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.getPageByURLId(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPageByURLId: $e\n');
}
[inline-code-end]