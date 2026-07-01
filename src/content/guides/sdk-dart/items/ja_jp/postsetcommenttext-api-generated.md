## パラメータ

| 名前 | 型 | ロケーション | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| commentId | string | path | はい |  |
| broadcastId | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: `SetCommentTextResponse`

## 例

[inline-code-attrs-start title = 'postSetCommentText 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // 文字列 | 
final commentId = commentId_example; // 文字列 | 
final setCommentTextParams = SetCommentTextParams(); // SetCommentTextParams | 
final broadcastId = broadcastId_example; // 文字列 | 
final sso = sso_example; // 文字列 | 

try {
    final result = api_instance.postSetCommentText(tenantId, commentId, setCommentTextParams, PostSetCommentTextOptions(broadcastId: broadcastId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->postSetCommentText: $e\n');
}
[inline-code-end]