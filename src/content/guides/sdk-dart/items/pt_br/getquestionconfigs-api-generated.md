## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Resposta

Retorna: `GetQuestionConfigsResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getQuestionConfigs'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave de API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar o prefixo (por exemplo, Bearer) da chave de API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getQuestionConfigs(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getQuestionConfigs: $e\n');
}
[inline-code-end]