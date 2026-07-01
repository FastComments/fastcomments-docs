## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |

## Resposta

Retorna: `GetUserResponse`

## Exemplo

[inline-code-attrs-start title = 'getUser Exemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configure a autorização da chave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// Descomente abaixo para configurar o prefixo (ex.: Bearer) para a chave API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getUser(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUser: $e\n');
}
[inline-code-end]

---