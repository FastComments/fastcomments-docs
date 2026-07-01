ページ上の過去のコメント投稿者で、現在オンラインではないもの。displayName でソートされます。  
`/users/online` をすべて使用した後に、「Members」セクションを表示するために使用します。  
commenterName に対するカーソルページング: サーバーは部分的な `{tenantId, urlId, commenterName}` インデックスを `afterName` から `$gt` を使用して前方に走査し、`$skip` コストはかかりません。

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページ URL 識別子（サーバー側でクリーンアップ）。 |
| afterName | string | query | No | カーソル: 前回のレスポンスから `nextAfterName` を渡す。 |
| afterUserId | string | query | No | カーソルのタイブレーカ: 前回のレスポンスから `nextAfterUserId` を渡す。`afterName` が設定されている場合に必要で、名前が同じでエントリが落ちないようにする。 |

## レスポンス

Returns: `PageUsersOfflineResponse`

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | ページ URL 識別子（サーバー側でクリーンアップ）。
final afterName = afterName_example; // String | カーソル: 前回のレスポンスから nextAfterName を渡す。
final afterUserId = afterUserId_example; // String | カーソルのタイブレーカ: 前回のレスポンスから nextAfterUserId を渡す。afterName が設定されている場合に必要で、名前が同じでエントリが落ちないようにする。

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]