List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозирни курсор за пагинацију који се враћа као `nextCursor` из претходног захтева. Везан за исти `sortBy`. |
| limit | integer | query | No | 1..200, подразумевано 50 |
| q | string | query | No | Опционални филтер префикса наслова без разликовања величине слова. |
| sortBy | string | query | No | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (абецедно). |
| hasComments | boolean | query | No | Ако је истинито, враћа само странице са бар једним коментаром. |

## Response

Враћа: `GetPublicPagesResponse`

## Пример

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Непрозирни курсор за пагинацију који се враћа као `nextCursor` из претходног захтева. Везан за исти `sortBy`.
final limit = 56; // int | 1..200, подразумевано 50
final q = q_example; // String | Опционални филтер префикса наслова без разликовања величине слова.
final sortBy = ; // PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (абецедно).
final hasComments = true; // bool | Ако је истинито, враћа само странице са бар једним коментаром.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]