## Parámetros

| Nombre | Tipo | Ubicación | Requerido | Descripción |
|--------|------|-----------|-----------|-------------|
| tenantId | string | query | Sí |  |
| urlId | string | query | No | Utilizado para determinar si la página actual está suscrita. |
| pageSize | integer | query | No |  |
| afterId | string | query | No |  |
| includeContext | boolean | query | No |  |
| afterCreatedAt | integer | query | No |  |
| unreadOnly | boolean | query | No |  |
| dmOnly | boolean | query | No |  |
| noDm | boolean | query | No |  |
| includeTranslations | boolean | query | No |  |
| includeTenantNotifications | boolean | query | No |  |
| sso | string | query | No |  |

## Respuesta

Devuelve: `GetMyNotificationsResponse`

## Ejemplo

[inline-code-attrs-start title = 'getUserNotifications Ejemplo'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final urlId = urlId_example; // String | Utilizado para determinar si la página actual está suscrita.
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