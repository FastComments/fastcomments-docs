## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|-----------|
| tenantId | string | query | Sim |  |

## Resposta

Retorna: `BulkCreateHashTagsResponse`

## Exemplo

[inline-code-attrs-start title = 'addHashTagsBulk Exemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave de API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar o prefixo (ex.: Bearer) da chave de API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final bulkCreateHashTagsBody = BulkCreateHashTagsBody(); // BulkCreateHashTagsBody | 

try {
    final result = api_instance.addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addHashTagsBulk: $e\n');
}
[inline-code-end]

---