## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Sim |  |
| skip | number | query | Não |  |

## Resposta

Retorna: `GetEmailTemplatesResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getEmailTemplates'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave de API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// uncomment below to setup prefix (e.g. Bearer) for API key, if needed
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getEmailTemplates(tenantId, skip);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getEmailTemplates: $e\n');
}
[inline-code-end]