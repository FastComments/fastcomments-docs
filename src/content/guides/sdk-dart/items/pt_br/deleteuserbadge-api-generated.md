## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Resposta

Retorna: `APIEmptySuccessResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo deleteUserBadge'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave da API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar o prefixo (ex.: Bearer) para a chave da API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.deleteUserBadge(tenantId, id);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteUserBadge: $e\n');
}
[inline-code-end]

---