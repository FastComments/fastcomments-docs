List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Sim |  |
| cursor | string | query | Não | Cursor de paginação opaco retornado como `nextCursor` de uma solicitação anterior. Vinculado ao mesmo `sortBy`. |
| limit | integer | query | Não | 1..200, padrão 50 |
| q | string | query | Não | Filtro opcional de prefixo de título sem distinção entre maiúsculas e minúsculas. |
| sortBy | string | query | Não | Ordem de classificação. `updatedAt` (padrão, mais recente primeiro), `commentCount` (maior número de comentários primeiro) ou `title` (alfabética). |
| hasComments | boolean | query | Não | Se verdadeiro, retorna apenas páginas com ao menos um comentário. |

## Resposta

Retorna: `GetPublicPagesResponse`

## Exemplo

[inline-code-attrs-start title = 'Exemplo getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Cursor de paginação opaco retornado como `nextCursor` de uma solicitação anterior. Vinculado ao mesmo `sortBy`.
final limit = 56; // int | 1..200, padrão 50
final q = q_example; // String | Filtro opcional de prefixo de título sem distinção entre maiúsculas e minúsculas.
final sortBy = ; // PagesSortBy | Ordem de classificação. `updatedAt` (padrão, mais recente primeiro), `commentCount` (maior número de comentários primeiro) ou `title` (alfabética).
final hasComments = true; // bool | Se verdadeiro, retorna apenas páginas com ao menos um comentário.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]