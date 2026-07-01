## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| editKey | string | query | No |  |

## Odgovor

Vraća: `VoteDeleteResponse`

## Primer

[inline-code-attrs-start title = 'deleteVote Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite dole da postavite prefiks (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final editKey = editKey_example; // String | 

try {
    final result = api_instance.deleteVote(tenantId, id, editKey);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteVote: $e\n');
}
[inline-code-end]