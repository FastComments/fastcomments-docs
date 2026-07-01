## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|--------------|-----------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |

## Resposta

Returns: `APIGetUserBadgeProgressResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserBadgeProgressById'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar o prefixo (ex.: Bearer) para a chave API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUserBadgeProgressById(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadgeProgressById: $e\n');
}
[inline-code-end]