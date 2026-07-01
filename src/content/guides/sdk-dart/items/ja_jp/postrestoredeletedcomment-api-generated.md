## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| broadcastId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## 応答

Returns: `APIEmptyResponse`

## 例

[inline-code-attrs-start title = 'postRestoreDeletedComment の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final broadcastId = broadcastId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.postRestoreDeletedComment(tenantId, commentId, PostRestoreDeletedCommentOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postRestoreDeletedComment: $e\n');
}
[inline-code-end]

---