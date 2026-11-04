頁面上目前不在線上的過往留言者。按 displayName 排序。
在用完 /users/online 之後,使用此端點來渲染「成員」區塊。
以 commenterName 進行游標分頁:伺服器透過 $gt 從 afterName 開始沿著部分索引 {tenantId, urlId, commenterName} 向後遍歷,沒有 $skip 的成本。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 頁面 URL 識別碼(會在伺服器端清理)。 |
| afterName | string | query | 否 | 游標:傳入上一個回應中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游標決勝欄位:傳入上一個回應中的 nextAfterUserId。當設定 afterName 時為必填,以避免同名項目被遺漏。 |

## 回應

回傳:`PageUsersOfflineResponse`

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 頁面 URL 識別碼(會在伺服器端清理)。
final afterName = afterName_example; // String | 游標:傳入上一個回應中的 nextAfterName。
final afterUserId = afterUserId_example; // String | 游標決勝欄位:傳入上一個回應中的 nextAfterUserId。當設定 afterName 時為必填,以避免同名項目被遺漏。

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]