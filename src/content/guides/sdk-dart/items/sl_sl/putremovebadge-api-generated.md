## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| badgeId | string | query | Yes |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Odgovor

Vrne: `RemoveUserBadgeResponse`

## Primer

[inline-code-attrs-start title = 'putRemoveBadge Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final badgeId = badgeId_example; // String | 
final userId = userId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.putRemoveBadge(tenantId, badgeId, PutRemoveBadgeOptions(userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putRemoveBadge: $e\n');
}
[inline-code-end]