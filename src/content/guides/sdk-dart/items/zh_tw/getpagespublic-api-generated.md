List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 先前請求返回的不透明分頁游標，作為 `nextCursor`。與相同的 `sortBy` 相關聯。 |
| limit | integer | query | 否 | 1..200，預設 50 |
| q | string | query | 否 | 可選的大小寫不敏感標題前綴過濾。 |
| sortBy | string | query | 否 | 排序依據。`updatedAt`（預設，最新優先），`commentCount`（評論數最多優先），或 `title`（字母順序）。 |
| hasComments | boolean | query | 否 | 若為 true，僅返回至少有一條評論的頁面。 |

## Response

返回： `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | 先前請求返回的不透明分頁游標，作為 `nextCursor`。與相同的 `sortBy` 相關聯。
final limit = 56; // int | 1..200，預設 50
final q = q_example; // String | 可選的大小寫不敏感標題前綴過濾。
final sortBy = ; // PagesSortBy | 排序依據。`updatedAt`（預設，最新優先），`commentCount`（評論數最多優先），或 `title`（字母順序）。
final hasComments = true; // bool | 若為 true，僅返回至少有一條評論的頁面。

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]

---