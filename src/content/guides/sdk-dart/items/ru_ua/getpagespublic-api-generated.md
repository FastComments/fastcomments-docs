---
Список страниц для арендатора. Используется настольным клиентом FChat для заполнения списка комнат.  
Требуется, чтобы `enableFChat` был установлен в true в разрешённой пользовательской конфигурации для каждой страницы.  
Страницы, требующие SSO, фильтруются по групповому доступу запрашивающего пользователя.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`. |
| limit | integer | query | No | 1..200, по умолчанию 50 |
| q | string | query | No | Необязательный фильтр префикса названия без учёта регистра. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, новые первые), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (по алфавиту). |
| hasComments | boolean | query | No | Если true, возвращать только страницы, содержащие хотя бы один комментарий. |

## Response

Возвращает: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Связан с тем же `sortBy`.
final limit = 56; // int | 1..200, по умолчанию 50
final q = q_example; // String | Необязательный фильтр префикса названия без учёта регистра.
final sortBy = ; // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, новые первые), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (по алфавиту).
final hasComments = true; // bool | Если true, возвращать только страницы, содержащие хотя бы один комментарий.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]

---