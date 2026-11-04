Lista páginas para um tenant. Usada pelo cliente desktop FChat para popular sua lista de salas.
Requer que `enableFChat` seja true na configuração customizada resolvida para cada página.
Páginas que exigem SSO são filtradas de acordo com o acesso a grupos do usuário solicitante.

## Parâmetros

| Nome | Tipo | Local | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| cursor | string | query | Não | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | Não | 1..200, padrão 50 |
| q | string | query | Não | Filtro opcional de prefixo de título, sem distinção entre maiúsculas e minúsculas. |
| sortBy | string | query | Não | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentados primeiro) ou `title` (alfabética). |
| hasComments | boolean | query | Não | Se true, retorna apenas páginas com pelo menos um comentário. |

## Resposta

Retorna: `GetPublicPagesResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Cursor opaco de paginação retornado como `nextCursor` de uma requisição anterior. Vinculado ao mesmo `sortBy`.
final limit = 56; // int | 1..200, padrão 50
final q = q_example; // String | Filtro opcional de prefixo de título, sem distinção entre maiúsculas e minúsculas.
final sortBy = ; // PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recentes primeiro), `commentCount` (mais comentados primeiro) ou `title` (alfabética).
final hasComments = true; // bool | Se true, retorna apenas páginas com pelo menos um comentário.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]