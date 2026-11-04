Выводит список страниц для тенанта. Используется десктоп-клиентом FChat для заполнения списка комнат.
Требует `enableFChat` = true в резолвнутом пользовательском конфиге для каждой страницы.
Страницы, требующие SSO, фильтруются по групповому доступу запрашивающего пользователя.

## Параметры

| Имя | Тип | Расположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Нет | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`. |
| limit | integer | query | Нет | 1..200, по умолчанию 50 |
| q | string | query | Нет | Необязательный фильтр по префиксу заголовка без учёта регистра. |
| sortBy | string | query | Нет | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев) или `title` (по алфавиту). |
| hasComments | boolean | query | Нет | Если true, возвращать только страницы, у которых есть хотя бы один комментарий. |

## Ответ

Возвращает: `GetPublicPagesResponse`

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Непрозрачный курсор пагинации, возвращённый как `nextCursor` из предыдущего запроса. Привязан к тому же `sortBy`.
final limit = 56; // int | 1..200, по умолчанию 50
final q = q_example; // String | Необязательный фильтр по префиксу заголовка без учёта регистра.
final sortBy = ; // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала с наибольшим числом комментариев) или `title` (по алфавиту).
final hasComments = true; // bool | Если true, возвращать только страницы, у которых есть хотя бы один комментарий.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]