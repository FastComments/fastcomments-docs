## Параметры

| Имя | Тип | Location | Обязательно | Описание |
|------|------|----------|--------------|----------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да |  |
| usernameStartsWith | string | query | Нет |  |
| mentionGroupIds | array | query | Нет |  |
| sso | string | query | Нет |  |
| searchSection | string | query | Нет |  |

## Ответ

Возвращает: `SearchUsersResult`

## Пример

[inline-code-attrs-start title = 'Пример searchUsers'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final usernameStartsWith = usernameStartsWith_example; // String | 
final mentionGroupIds = []; // List<String> | 
final sso = sso_example; // String | 
final searchSection = searchSection_example; // String | 

try {
    final result = api_instance.searchUsers(tenantId, urlId, SearchUsersOptions(usernameStartsWith: usernameStartsWith, mentionGroupIds: mentionGroupIds, sso: sso, searchSection: searchSection));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->searchUsers: $e\n');
}
[inline-code-end]