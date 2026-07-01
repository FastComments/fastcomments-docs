## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Svar

Returnerer: `GetPagesAPIResponse`

## Eksempel

[inline-code-attrs-start title = 'getPages Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøgle autorisation: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// fjern kommentaren nedenfor for at opsætte præfiks (f.eks. Bearer) for API-nøgle, hvis nødvendigt
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 

try {
    final result = api_instance.getPages(tenantId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getPages: $e\n');
}
[inline-code-end]