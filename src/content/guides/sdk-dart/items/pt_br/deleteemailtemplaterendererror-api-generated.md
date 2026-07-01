## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| errorId | string | path | Sim |  |

## Resposta

Retorna: `APIEmptyResponse`

## Exemplo

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Exemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave de API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar o prefixo (por exemplo, Bearer) para a chave da API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final errorId = errorId_example; // String | 

try {
    final result = api_instance.deleteEmailTemplateRenderError(tenantId, id, errorId);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteEmailTemplateRenderError: $e\n');
}
[inline-code-end]

---