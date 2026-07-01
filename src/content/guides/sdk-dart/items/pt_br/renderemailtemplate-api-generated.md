## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| locale | string | query | Não |  |

## Resposta

Retorna: `RenderEmailTemplateResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo renderEmailTemplate'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave de API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar o prefixo (ex.: Bearer) da chave de API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final renderEmailTemplateBody = RenderEmailTemplateBody(); // RenderEmailTemplateBody | 
final locale = locale_example; // String | 

try {
    final result = api_instance.renderEmailTemplate(tenantId, renderEmailTemplateBody, locale);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->renderEmailTemplate: $e\n');
}
[inline-code-end]