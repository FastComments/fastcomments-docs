Past commenters on the page who are NOT currently online. Sorted by displayName.  
過去在此頁面上發表過評論，但目前未在線的評論者。依 displayName 排序。  

Use this after exhausting /users/online to render a "Members" section.  
在耗盡 /users/online 之後使用，以呈現「Members」區段。  

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
在 commenterName 上使用游標分頁：伺服器從 afterName 起透過 $gt 逐步走訪部分 {tenantId, urlId, commenterName} 索引，無需 $skip 成本。  

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別碼（在伺服器端清理後）。 |
| afterName | string | query | No | 游標：傳遞前一次回應中的 nextAfterName。 |
| afterUserId | string | query | No | 游標平手打破者：傳遞前一次回應中的 nextAfterUserId。當 afterName 被設定時必填，以防名稱相同導致條目被遺漏。 |

## Response

Returns: `PageUsersOfflineResponse`

## Example

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 頁面 URL 識別碼（在伺服器端清理後）。
final afterName = afterName_example; // String | 游標：傳遞前一次回應中的 nextAfterName。
final afterUserId = afterUserId_example; // String | 游標平手打破者：傳遞前一次回應中的 nextAfterUserId。當 afterName 被設定時必填，以防名稱相同導致條目被遺漏。

try {
    final result = api_instance.getOfflineUsers(tenantId, urlId, GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOfflineUsers: $e\n');
}
[inline-code-end]

---