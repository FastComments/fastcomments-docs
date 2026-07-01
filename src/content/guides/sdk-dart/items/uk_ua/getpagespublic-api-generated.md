List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| cursor | string | query | Ні | Непрозорий курсор пагінації, який повертається як `nextCursor` у попередньому запиті. Прив’язаний до того ж `sortBy`. |
| limit | integer | query | Ні | 1..200, за замовчуванням 50 |
| q | string | query | Ні | Необов’язковий фільтр за префіксом назви без урахування регістру. |
| sortBy | string | query | Ні | Порядок сортування. `updatedAt` (за замовчуванням, найновіші першими), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів), або `title` (за алфавітом). |
| hasComments | boolean | query | Ні | Якщо true, повертаються лише сторінки, що мають хоча б один коментар. |

## Response

Повертає: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'getPagesPublic Приклад'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Непрозорий курсор пагінації, який повертається як `nextCursor` у попередньому запиті. Прив’язаний до того ж `sortBy`.
final limit = 56; // int | 1..200, за замовчуванням 50
final q = q_example; // String | Необов’язковий фільтр за префіксом назви без урахування регістру.
final sortBy = ; // PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, найновіші першими), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів), або `title` (за алфавітом).
final hasComments = true; // bool | Якщо true, повертаються лише сторінки, що мають хоча б один коментар.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]