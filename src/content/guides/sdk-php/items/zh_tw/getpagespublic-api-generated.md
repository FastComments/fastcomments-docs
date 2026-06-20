列出租戶的頁面。被 FChat 桌面用戶端用來填充其房間列表。
要求在每個頁面的已解析自訂設定中，`enableFChat` 必須為 true。
需要 SSO 的頁面會根據請求使用者的群組存取權限進行篩選。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 先前請求回傳的 Opaque 分頁游標，作為 `nextCursor`。與相同的 `sortBy` 綁定。 |
| limit | integer | query | 否 | 1..200，預設 50 |
| q | string | query | 否 | 可選的不區分大小寫的標題前綴篩選。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`（預設，最新的在前）、`commentCount`（評論數最多在前）或 `title`（字母排序）。 |
| hasComments | boolean | query | 否 | 如果為 true，僅回傳至少有一則評論的頁面。 |

## 回應

回傳: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 http client，請傳遞實作了 `GuzzleHttp\ClientInterface` 的 client。
    // 此為選用，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | 先前請求回傳的 Opaque 分頁游標，作為 `nextCursor`。與相同的 `sortBy` 綁定。
$limit = 56; // int | 1..200，預設 50
$q = 'q_example'; // string | 可選的不區分大小寫的標題前綴篩選。
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | 排序方式。`updatedAt`（預設，最新的在前），`commentCount`（評論最多的在前），或 `title`（字母順序）。
$has_comments = True; // bool | 如果為 true，僅回傳至少有一則評論的頁面。

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]