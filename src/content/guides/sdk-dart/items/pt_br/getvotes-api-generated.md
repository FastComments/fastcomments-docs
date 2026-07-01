## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |

## Resposta

Retorna: `GetVotesResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getVotes'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.getVotes(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getVotes: $e\n');
}
[inline-code-end]