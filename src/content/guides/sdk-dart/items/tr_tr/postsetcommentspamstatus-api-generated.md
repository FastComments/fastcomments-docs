## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | path | Evet |  |
| spam | boolean | query | Hayır |  |
| permNotSpam | boolean | query | Hayır |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: `APIEmptyResponse`

## Örnek

[inline-code-attrs-start title = 'postSetCommentSpamStatus Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final spam = true; // bool | 
final permNotSpam = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final s0 = sso_example; // String | 

try {
    final result = api_instance.postSetCommentSpamStatus(tenantId, commentId, PostSetCommentSpamStatusOptions(spam: spam, permNotSpam: permNotSpam, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentSpamStatus: $e\n');
}
[inline-code-end]

---