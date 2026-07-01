## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| fromName | string | query | Yes |  |

## Resposta

Retorna: `APIEmptyResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo sendInvite'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave de API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para definir um prefixo (ex.: Bearer) para a chave de API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final fromName = fromName_example; // String | 

try {
    final result = api_instance.sendInvite(tenantId, id, fromName);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->sendInvite: $e\n');
}
[inline-code-end]