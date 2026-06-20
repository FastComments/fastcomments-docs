目前線上觀看該頁面的使用者：指 websocket 會話目前已訂閱該頁面的人。
回傳 anonCount + totalCount（房間範圍內的訂閱者數，包括我們不列舉的匿名觀眾）。

## 參數

| 名稱 | 類型 | 位置 | 必要 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別碼（伺服器端清理）。 |
| afterName | string | query | No | 游標：傳入前一次回應的 nextAfterName。 |
| afterUserId | string | query | No | 游標決勝項：傳入前一次回應的 nextAfterUserId。在設置 afterName 時此參數為必要，以免名稱相同時導致條目遺失。 |

## 回應

回傳: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## 範例

[inline-code-attrs-start title = 'getOnlineUsers 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 頁面 URL 識別碼（伺服器端清理）。
$after_name = 'after_name_example'; // string | 游標：傳入前一次回應的 nextAfterName。
$after_user_id = 'after_user_id_example'; // string | 游標決勝項：傳入前一次回應的 nextAfterUserId。在設置 afterName 時此參數為必要，以免名稱相同時導致條目遺失。

try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $after_name, $after_user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]