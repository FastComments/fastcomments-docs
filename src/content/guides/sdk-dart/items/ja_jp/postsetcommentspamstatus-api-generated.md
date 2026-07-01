## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| spam | boolean | query | No |  |
| permNotSpam | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 応答

返却: `APIEmptyResponse`

## 例

[inline-code-attrs-start title = 'postSetCommentSpamStatus の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final spam = true; // bool | 
final permNotSpam = true; // bool | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postSetCommentSpamStatus(tenantId, commentId, PostSetCommentSpamStatusOptions(spam: spam, permNotSpam: permNotSpam, broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentSpamStatus: $e\n');
}
[inline-code-end]