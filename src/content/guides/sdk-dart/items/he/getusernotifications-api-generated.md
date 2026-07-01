## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| urlId | string | query | לא | משמש לקביעת האם הדף הנוכחי מנוי. |
| pageSize | integer | query | לא |  |
| afterId | string | query | לא |  |
| includeContext | boolean | query | לא |  |
| afterCreatedAt | integer | query | לא |  |
| unreadOnly | boolean | query | לא |  |
| dmOnly | boolean | query | לא |  |
| noDm | boolean | query | לא |  |
| includeTranslations | boolean | query | לא |  |
| includeTenantNotifications | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: `GetMyNotificationsResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת getUserNotifications'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final urlId = urlId_example; // String | משמש לקביעת האם הדף הנוכחי מנוי.
final pageSize = 56; // int |
final afterId = afterId_example; // String |
final includeContext = true; // bool |
final afterCreatedAt = 789; // int |
final unreadOnly = true; // bool |
final dmOnly = true; // bool |
final noDm = true; // bool |
final includeTranslations = true; // bool |
final includeTenantNotifications = true; // bool |
final sso = sso_example; // String |

try {
    final result = api_instance.getUserNotifications(tenantId, GetUserNotificationsOptions(urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sno));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserNotifications: $e\n');
}
[inline-code-end]