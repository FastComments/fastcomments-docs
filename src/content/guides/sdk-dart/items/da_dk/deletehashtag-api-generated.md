## Parametre

| Navn | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| tag | string | path | Ja |  |

## Svar

Returnerer: `APIEmptyResponse`

## Eksempel

[inline-code-attrs-start title = 'deleteHashTag Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurer API-nøgle autorisation: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// fjern kommentaren nedenfor for at indstille præfiks (fx Bearer) for API-nøglen, hvis nødvendigt
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final tag = tag_example; // String | 
final deleteHashTagRequestBody = DeleteHashTagRequestBody(); // DeleteHashTagRequestBody | 

try {
    final result = api_instance.deleteHashTag(tenantId, tag, deleteHashTagRequestBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteHashTag: $e\n');
}
[inline-code-end]