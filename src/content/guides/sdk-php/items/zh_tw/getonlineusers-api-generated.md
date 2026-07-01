Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
已訂閱該頁面之 websocket 連線使用者，即目前線上檢視頁面的觀眾。

Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
返回 anonCount + totalCount（整個房間的訂閱者，包括我們不列舉的匿名檢視者）。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別碼（在伺服器端已清理）。 |
| afterName | string | query | No | 游標：傳入先前回應中的 nextAfterName。 |
| afterUserId | string | query | No | 游標平鋪決定者：傳入先前回應中的 nextAfterUserId。當設定 afterName 時必填，以避免因名稱相同而遺漏條目。 |

## Response

Returns: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOnlineResponse.php)

## Example

[inline-code-attrs-start title = '取得線上使用者 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 頁面 URL 識別碼（在伺服器端已清理）。
$options = [
    'after_name' => 'after_name_example', // string | 游標：傳入先前回應中的 nextAfterName。
    'after_user_id' => 'after_user_id_example', // string | 游標平鋪決定者：傳入先前回應中的 nextAfterUserId。當設定 afterName 時必填，以避免因名稱相同而遺漏條目。
];


try {
    $result = $apiInstance->getOnlineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOnlineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---