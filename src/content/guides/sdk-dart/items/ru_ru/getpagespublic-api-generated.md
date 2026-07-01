List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|--------------|----------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Неявный курсор пагинации, возвращаемый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`. |
| limit | integer | query | No | 1..200, по умолчанию 50 |
| q | string | query | No | Необязательный нечувствительный к регистру фильтр префикса названия. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, от новых к старым), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (в алфавитном порядке). |
| hasComments | boolean | query | No | Если true, возвращать только страницы, содержащие хотя бы один комментарий. |

## Ответ

Возвращает: `GetPublicPagesResponse`

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Неявный курсор пагинации, возвращаемый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`.
final limit = 56; // int | 1..200, по умолчанию 50
final q = q_example; // String | Необязательный нечувствительный к регистру фильтр префикса названия.
final sortBy = ; // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, от новых к старым), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (в алфавитном порядке).
final hasComments = true; // bool | Если true, возвращать только страницы, содержащие хотя бы один комментарий.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]