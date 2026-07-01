## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|------|------|
| tenantId | string | クエリ | はい |  |
| id | string | パス | はい |  |
| contextUserId | string | クエリ | いいえ |  |
| isLive | boolean | クエリ | いいえ |  |

## レスポンス

戻り値: `DeleteCommentResult`

## 例

[inline-code-attrs-start title = 'deleteComment の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 下記のコメントアウトを解除して、必要に応じて API キーのプレフィックス (例: Bearer) を設定
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final contextUserId = contextUserId_example; // String | 
final isLive = true; // bool | 

try {
    final result = api_instance.deleteComment(tenantId, id, DeleteCommentOptions(contextUserId: contextUserId, isLive: isLive));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->deleteComment: $e\n');
}
[inline-code-end]