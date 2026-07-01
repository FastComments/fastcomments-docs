## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | query | Да |  |
| commentId | string | path | Да |  |
| banEmail | boolean | query | Нет |  |
| banEmailDomain | boolean | query | Нет |  |
| banIP | boolean | query | Нет |  |
| deleteAllUsersComments | boolean | query | Нет |  |
| bannedUntil | string | query | Нет |  |
| isShadowBan | boolean | query | Нет |  |
| updateId | string | query | Нет |  |
| banReason | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: `BanUserFromCommentResult`

## Пример

[inline-code-attrs-start title = 'postBanUserFromComment Пример'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final banEmail = true; // bool | 
final banEmailDomain = true; // bool | 
final banIP = true; // bool | 
final deleteAllUsersComments = true; // bool | 
final bannedUntil = bannedUntil_example; // String | 
final isShadowBan = true; // bool | 
final updateId = updateId_example; // String | 
final banReason = banReason_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postBanUserFromComment(tenantId, commentId, PostBanUserFromCommentOptions(banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postBanUserFromComment: $e\n');
}
[inline-code-end]