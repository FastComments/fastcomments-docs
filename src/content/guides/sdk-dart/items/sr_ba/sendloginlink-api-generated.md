## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| redirectURL | string | query | No |  |

## Response

Returns: `APIEmptyResponse`

## Example

[inline-code-attrs-start title = 'Primjer sendLoginLink'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurišite autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// otkomentarišite dole da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
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