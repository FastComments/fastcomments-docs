## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | path | Evet |  |
| reviewed | boolean | query | Hayır |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: `APIEmptyResponse`

## Örnek

[inline-code-attrs-start title = 'postSetCommentReviewStatus Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final reviewed = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentReviewStatus(tenantId, commentId, PostSetCommentReviewStatusOptions(reviewed: reviewed, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentReviewStatus: $e\n');
}
[inline-code-end]