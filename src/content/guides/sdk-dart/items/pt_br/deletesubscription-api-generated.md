## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Resposta

Retorna: `DeleteSubscriptionAPIResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo deleteSubscription'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar prefixo (ex.: Bearer) para a chave API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final userId = userId_example; // String | 

try {
    final result = api_instance.deleteSubscription(tenantId, id, userId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteSubscription: $e\n');
}
[inline-code-end]

---