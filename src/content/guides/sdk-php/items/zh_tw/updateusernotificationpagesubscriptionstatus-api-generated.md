啟用或停用某個頁面的通知。當使用者訂閱了某個頁面時，新的根層級留言會產生通知，並且

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 是 |  |
| url | string | query | 是 |  |
| pageTitle | string | query | 是 |  |
| subscribedOrUnsubscribed | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationPageSubscriptionStatusResponse.php)

## 範例

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 http client，請傳入實作了 `GuzzleHttp\ClientInterface` 的 client。
    // 這是選填的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$url = 'url_example'; // string
$page_title = 'page_title_example'; // string
$subscribed_or_unsubscribed = 'subscribed_or_unsubscribed_example'; // string
$sso = 'sso_example'; // string


try {
    $result = $apiInstance->updateUserNotificationPageSubscriptionStatus($tenant_id, $url_id, $url, $page_title, $subscribed_or_unsubscribed, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationPageSubscriptionStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]