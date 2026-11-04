Приказ корисничких информација за закупца (tenant). За дате userIds, враћа приказне информације из User / SSOUser.
Користи га виџет за коментаре да обогати кориснике који су управо појавили путем догађаја присуства.
Без контекста странице: приватност се спроводи једнолико (приватни профили су маскирани).

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| ids | string | query | Да | userIds раздвојени зарезом. |

## Одговор

Враћа: `PageUsersInfoResponse`

## Пример

[inline-code-attrs-start title = 'getUsersInfo пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | userIds раздвојени зарезом.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]