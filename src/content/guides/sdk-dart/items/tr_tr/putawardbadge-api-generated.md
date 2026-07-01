## Parametreler

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| badgeId | string | query | Evet |  |
| userId | string | query | Hayır |  |
| commentId | string | query | Hayır |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: `AwardUserBadgeResponse`

## Örnek

[inline-code-attrs-start title = 'putAwardBadge Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
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
    final result = api_instance.putAwardBadge(tenantId, badgeId, PutAwardBadgeOptions(userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->putAwardBadge: $e\n');
}
[inline-code-end]