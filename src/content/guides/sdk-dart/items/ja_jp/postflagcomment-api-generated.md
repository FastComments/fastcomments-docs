## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| commentId | string | パス | はい |  |
| broadcastId | string | クエリ | いいえ |  |
| sso | string | クエリ | いいえ |  |

## 応答

戻り値: `APIEmptyResponse`

## 例

[inline-code-attrs-start title = 'postFlagComment の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postFlagComment(tenantId, commentId, PostFlagCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postFlagComment: $e\n');
}
[inline-code-end]