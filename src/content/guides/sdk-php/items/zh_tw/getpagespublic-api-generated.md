列出租戶的頁面。由 FChat 桌面客戶端用於填充其房間列表。
每個頁面的已解析自訂設定中必須將 `enableFChat` 設為 true。
需要 SSO 的頁面會根據請求使用者的群組存取權限進行篩選。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 由先前請求以 `nextCursor` 回傳的不透明分頁游標。與相同的 `sortBy` 綁定。 |
| limit | integer | query | 否 | 1..200,預設 50 |
| q | string | query | 否 | 選填的、不區分大小寫的標題前綴篩選條件。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`(預設,最新的在前)、`commentCount`(評論數最多的在前)或 `title`(按字母順序)。 |
| hasComments | boolean | query | 否 | 若為 true,僅回傳至少有一則評論的頁面。 |

## 回應

回傳:[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 http 客戶端,請傳入實作了 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是選填的,預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | 由先前請求以 `nextCursor` 回傳的不透明分頁游標。與相同的 `sortBy` 綁定。
    'limit' => 56, // int | 1..200,預設 50
    'q' => 'q_example', // string | 選填的、不區分大小寫的標題前綴篩選條件。
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | 排序方式。`updatedAt`(預設,最新的在前)、`commentCount`(評論數最多的在前)或 `title`(按字母順序)。
    'has_comments' => True, // bool | 若為 true,僅回傳至少有一則評論的頁面。
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]