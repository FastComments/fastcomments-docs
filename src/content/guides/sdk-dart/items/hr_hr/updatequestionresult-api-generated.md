## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |

## Odgovor

Vraća: `APIEmptyResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer updateQuestionResult'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Konfigurirajte autorizaciju API ključa: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// odkomentirajte dolje kako biste postavili prefiks (npr. Bearer) za API ključ, po potrebi
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final updateQuestionResultBody = UpdateQuestionResultBody(); // UpdateQuestionResultBody | 

try {
    final result = api_instance.updateQuestionResult(tenantId, id, updateQuestionResultBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->updateQuestionResult: $e\n');
}
[inline-code-end]