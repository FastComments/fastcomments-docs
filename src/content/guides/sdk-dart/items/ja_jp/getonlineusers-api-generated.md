Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページ URL 識別子（サーバー側でクリーンアップ済み）。 |
| afterName | string | query | No | カーソル: 前回のレスポンスから nextAfterName を渡します。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡します。afterName が設定されている場合、名前のタイがエントリを除外しないように必須です。 |

## Response

返却: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'getOnlineUsers の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final urlId = urlId_example; // String | ページ URL 識別子（サーバー側でクリーンアップ済み）。
final afterName = afterName_example; // String | カーソル: 前回のレスポンスから nextAfterName を渡します。
final afterUserId = afterUserId_example; // String | カーソルのタイブレーカー: 前回のレスポンスから nextAfterUserId を渡します。afterName が設定されている場合、名前のタイがエントリを除外しないように必須です。

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]