List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 不透明的分頁游標，從先前的請求中作為 `nextCursor` 返回。與相同的 `sortBy` 相關聯。 |
| limit | integer | query | No | 1..200，預設 50 |
| q | string | query | No | 可選的大小寫不敏感的標題前綴過濾器。 |
| sortBy | string | query | No | 排序方式。`updatedAt`（預設，最新優先），`commentCount`（評論數最多優先），或 `title`（字母順序）。 |
| hasComments | boolean | query | No | 若為 true，僅返回至少有一條評論的頁面。 |

## 回應

回傳: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 客戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 字串
$options = [
    'cursor' => 'cursor_example', // 字串 | 不透明的分頁游標，從先前的請求中作為 `nextCursor` 返回。與相同的 `sortBy` 相關聯。
    'limit' => 56, // 整數 | 1..200，預設 50
    'q' => 'q_example', // 字串 | 可選的大小寫不敏感的標題前綴過濾器。
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | 排序方式。`updatedAt`（預設，最新優先），`commentCount`（評論數最多優先），或 `title`（字母順序）。
    'has_comments' => True, // 布林值 | 若為 true，僅返回至少有一條評論的頁面。
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---