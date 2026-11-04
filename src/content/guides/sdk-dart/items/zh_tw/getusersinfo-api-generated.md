租戶的批次使用者資訊。給定 userIds，從 User / SSOUser 回傳顯示資訊。
供留言小工具用來豐富剛透過 presence 事件出現的使用者。
無頁面上下文：隱私權一律統一執行（私人設定檔會被遮蔽）。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| ids | string | query | 是 | 以逗號分隔的 userIds。 |

## 回應

回傳：`PageUsersInfoResponse`

## 範例

[inline-code-attrs-start title = 'getUsersInfo 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Comma-delimited userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]