req  
tenantId  
afterId  

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| afterId | string | query | Não |  |
| limit | integer | query | Não |  |
| tags | array | query | Não |  |
| sso | string | query | Não |  |
| isCrawler | boolean | query | Não |  |
| includeUserInfo | boolean | query | Não |  |

## Resposta

Retorna: `PublicFeedPostsResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getFeedPostsPublic'; type = ''; isFunctional = false; inline-code-attrs-end]  
[inline-code-start]  
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final afterId = afterId_example; // String | 
final limit = 56; // int | 
final tags = []; // List<String> | 
final sso = sso_example; // String | 
final isCrawler = true; // bool | 
final includeUserInfo = true; // bool | 

try {
    final result = api_instance.getFeedPostsPublic(tenantId, GetFeedPostsPublicOptions(afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getFeedPostsPublic: $e\n');
}
[inline-code-end]