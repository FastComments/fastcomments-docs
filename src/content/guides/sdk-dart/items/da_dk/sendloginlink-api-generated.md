## Parametre

| Navn | Type | Placering | Krævet | Beskrivelse |
|------|------|-----------|--------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| redirectURL | string | query | No |  |

## Svar

Returnerer: `APIEmptyResponse`

## Eksempel

[inline-code-attrs-start title = 'sendLoginLink Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøgle autorisation: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final redirectURL = redirectURL_example; // String | 

try {
    final result = api_instance.sendLoginLink(tenantId, id, redirectURL);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->sendLoginLink: $e\n');
}
[inline-code-end]

---