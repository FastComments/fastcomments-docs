Списъци с страници за даден tenant. Използва се от FChat десктоп клиента за попълване на списъка му със стаи.
Изисква `enableFChat` да е true в резолвнатата персонализирана конфигурация за всяка страница.
Страниците, изискващи SSO, се филтрират спрямо достъпа на групата на заявяващия потребител.

## Параметри

| Име | Тип | Местоположение | Задължителен | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозрачен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. |
| limit | integer | query | Не | 1..200, по подразбиране 50 |
| q | string | query | Не | Незадължителен филтър по префикс на заглавие, без значение на регистъра. |
| sortBy | string | query | Не | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи) или `title` (по азбучен ред). |
| hasComments | boolean | query | Не | Ако е true, връща само страници с поне един коментар. |

## Отговор

Връща: `GetPublicPagesResponse`

## Пример

[inline-code-attrs-start title = 'Пример за getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | Непрозрачен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`.
final limit = 56; // int | 1..200, по подразбиране 50
final q = q_example; // String | Незадължителен филтър по префикс на заглавие, без значение на регистъра.
final sortBy = ; // PagesSortBy | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи) или `title` (по азбучен ред).
final hasComments = true; // bool | Ако е true, връща само страници с поне един коментар.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]