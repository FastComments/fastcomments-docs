## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|--------------|-----------|
| tenantId | string | query | Sim |  |
| userId | string | query | Não |  |
| urlId | string | query | Não |  |
| fromCommentId | string | query | Não |  |
| viewed | boolean | query | Não |  |
| type | string | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: `GetNotificationsResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getNotifications'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO Configurar autorização da chave de API: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// descomente abaixo para configurar prefixo (ex. Bearer) para a chave de API, se necessário
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final urlId = urlId_example; // String | 
final fromCommentId = fromCommentId_example; // String | 
final viewed = true; // bool | 
final type = type_example; // String | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getNotifications(tenantId, GetNotificationsOptions(userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getNotifications: $e\n');
}
[inline-code-end]