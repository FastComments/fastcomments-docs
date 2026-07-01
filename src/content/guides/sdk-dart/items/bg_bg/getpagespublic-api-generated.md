List pages for a tenant. Used by the FChat desktop client to populate its room list.
Requires `enableFChat` to be true on the resolved custom config for each page.
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачен курсор за странициране, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. |
| limit | integer | query | No | 1..200, по подразбиране 50 |
| q | string | query | No | Незадължителен филтър за префикс на заглавието, без значение на главни/малки букви. |
| sortBy | string | query | No | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи), или `title` (по азбучен ред). |
| hasComments | boolean | query | No | Ако е true, връща само страници с поне един коментар. |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final cursor = cursor_example; // String | Непрозрачен курсор за странициране, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`.
final limit = 56; // int | 1..200, по подразбиране 50
final q = q_example; // String | Незадължителен филтър за префикс на заглавието, без значение на главни/малки букви.
final sortBy = ; // PagesSortBy | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи), или `title` (по азбучен ред).
final hasComments = true; // bool | Ако е true, връща само страници с поне един коментар.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]

---