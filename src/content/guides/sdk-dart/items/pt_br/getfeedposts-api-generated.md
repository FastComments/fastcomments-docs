requisição  
tenantId  
afterId  

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| afterId | string | query | Não |  |
| limit | integer | query | Não |  |
| tags | array | query | Não |  |

## Resposta

Retorna: `GetFeedPostsResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getFeedPosts'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar o prefixo (ex.: Bearer) para a chave API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final afterId = afterId_example; // String | 
final limit = 56; // int | 
final tags = []; // List<String> | 

try {
    final result = api_instance.getFeedPosts(tenantId, GetFeedPostsOptions(afterId: afterId, limit: limit, tags: tags));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getFeedPosts: $e\n');
}
[inline-code-end]