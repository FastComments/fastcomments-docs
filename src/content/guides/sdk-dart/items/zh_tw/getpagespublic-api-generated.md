列出租戶的頁面。由 FChat 桌面客戶端用於填充其房間列表。
要求每個頁面的已解析自訂設定中 `enableFChat` 為 true。
需要 SSO 的頁面會根據請求使用者的群組存取權進行篩選。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 由先前請求以 `nextCursor` 回傳的不透明分頁游標。與相同的 `sortBy` 綁定。 |
| limit | integer | query | 否 | 1..200,預設 50 |
| q | string | query | 否 | 可選的、不區分大小寫的標題前綴篩選條件。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`(預設,最新的在前)、`commentCount`(評論數最多的在前)或 `title`(按字母順序)。 |
| hasComments | boolean | query | 否 | 若為 true,僅回傳至少有一則評論的頁面。 |

## 回應

回傳:`GetPublicPagesResponse`

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | 由先前請求以 `nextCursor` 回傳的不透明分頁游標。與相同的 `sortBy` 綁定。
final limit = 56; // int | 1..200,預設 50
final q = q_example; // String | 可選的、不區分大小寫的標題前綴篩選條件。
final sortBy = ; // PagesSortBy | 排序方式。`updatedAt`(預設,最新的在前)、`commentCount`(評論數最多的在前)或 `title`(按字母順序)。
final hasComments = true; // bool | 若為 true,僅回傳至少有一則評論的頁面。

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]