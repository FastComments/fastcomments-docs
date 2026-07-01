## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| id | string | query | Sim |  |

## Resposta

Retorna: `GetV2PageReactUsersResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getV2PageReactUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final id = id_example; // String | 

try {
    final result = api_instance.getV2PageReactUsers(tenantId, urlId, id);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getV2PageReactUsers: $e\n');
}
[inline-code-end]