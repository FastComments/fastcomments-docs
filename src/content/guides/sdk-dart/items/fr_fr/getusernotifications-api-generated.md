## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| urlId | string | query | Non | Utilisé pour déterminer si la page actuelle est abonnée. |
| pageSize | integer | query | Non |  |
| afterId | string | query | Non |  |
| includeContext | boolean | query | Non |  |
| afterCreatedAt | integer | query | Non |  |
| unreadOnly | boolean | query | Non |  |
| dmOnly | boolean | query | Non |  |
| noDm | boolean | query | Non |  |
| includeTranslations | boolean | query | Non |  |
| includeTenantNotifications | boolean | query | Non |  |
| sso | string | query | Non |  |

## Response

Retourne : `GetMyNotificationsResponse`

## Exemple

[inline-code-attrs-start title = 'Exemple getUserNotifications'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Utilisé pour déterminer si la page actuelle est abonnée.
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
    final result = api_instance.getUserNotifications(tenantId, GetUserNotificationsOptions(urlId: urlId, pageSize: pageSize, afterId: afterId, includeContext: includeContext, afterCreatedAt: afterCreatedAt, unreadOnly: unreadOnly, dmOnly: dmOnly, noDm: noDm, includeTranslations: includeTranslations, includeTenantNotifications: includeTenantNotifications, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUserNotifications: $e\n');
}
[inline-code-end]

---