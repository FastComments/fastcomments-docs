目前線上檢視頁面的使用者：指其 websocket 連線目前訂閱該頁面的使用者。回傳 anonCount + totalCount（整個房間的訂閱者數量，包含我們未列舉的匿名檢視者）。

## Parameters

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|------|------|------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別碼（伺服器端清理後）。 |
| afterName | string | query | No | 游標：從先前的回應傳遞 nextAfterName。 |
| afterUserId | string | query | No | 游標平局斷點：從先前的回應傳遞 nextAfterUserId。當 afterName 被設定時必填，以免名稱相同的情況導致條目遺失。 |

## Response

Returns: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'getOnlineUsers 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 頁面 URL 識別碼（伺服器端清理後）。
final afterName = afterName_example; // String | 游標：從先前的回應傳遞 nextAfterName。
final afterUserId = afterUserId_example; // String | 游標平局斷點：從先前的回應傳遞 nextAfterUserId。當 afterName 被設定時必填，以免名稱相同的情況導致條目遺失。

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]