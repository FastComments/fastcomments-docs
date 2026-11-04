Сторінки списку для орендаря. Використовується десктоп-клієнтом FChat для заповнення списку кімнат.
Потребує `enableFChat` = true у вирішеному користувацькому конфізі для кожної сторінки.
Сторінки, що вимагають SSO, фільтруються відповідно до групового доступу користувача, який робить запит.

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| cursor | string | query | Ні | Непрозорий курсор пагінації, повернутий як `nextCursor` з попереднього запиту. Прив'язаний до того самого `sortBy`. |
| limit | integer | query | Ні | 1..200, за замовчуванням 50 |
| q | string | query | Ні | Необов'язковий фільтр за префіксом назви без урахування регістру. |
| sortBy | string | query | Ні | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом). |
| hasComments | boolean | query | Ні | Якщо true, повертати лише сторінки з принаймні одним коментарем. |

## Відповідь

Повертає: `GetPublicPagesResponse`

## Приклад

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Непрозорий курсор пагінації, повернутий як `nextCursor` з попереднього запиту. Прив'язаний до того самого `sortBy`.
final limit = 56; // int | 1..200, за замовчуванням 50
final q = q_example; // String | Необов'язковий фільтр за префіксом назви без урахування регістру.
final sortBy = ; // PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку з найбільшою кількістю коментарів) або `title` (за алфавітом).
final hasComments = true; // bool | Якщо true, повертати лише сторінки з принаймні одним коментарем.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]