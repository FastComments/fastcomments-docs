Наводи странице за тенајт. Користи FChat десктоп клијент да попуни листу својих соба.
Захтева да `enableFChat` буде true на резолвованој прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају према приступу групи тражењег корисника.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Опацни пагинациони курсор враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`. |
| limit | integer | query | Не | 1..200, подразумевано 50 |
| q | string | query | Не | Опциони филтер префикса наслова без обзира на велика и мала слова. |
| sortBy | string | query | Не | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (абецедно). |
| hasComments | boolean | query | Не | Ако је true, враћа само странице са бар једним коментаром. |

## Одговор

Враћа: `GetPublicPagesResponse`

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Опацни пагинациони курсор враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`.
final limit = 56; // int | 1..200, подразумевано 50
final q = q_example; // String | Опциони филтер префикса наслова без обзира на велика и мала слова.
final sortBy = ; // PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (абецедно).
final hasComments = true; // bool | Ако је true, враћа само странице са бар једним коментаром.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]