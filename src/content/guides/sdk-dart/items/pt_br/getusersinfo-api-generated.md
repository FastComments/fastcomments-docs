Informações em lote de usuários para um tenant. Dados os userIds, retorna informações de exibição de User / SSOUser.
Usado pelo widget de comentários para enriquecer usuários que acabaram de aparecer via um evento de presença.
Sem contexto de página: a privacidade é aplicada de forma uniforme (perfis privados são mascarados).

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|-------|-------------|-----------|
| tenantId | string | path | Sim |  |
| ids | string | query | Sim | userIds delimitados por vírgula. |

## Resposta

Retorna: `PageUsersInfoResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getUsersInfo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | userIds delimitados por vírgula.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]