## 參數

| 名稱 | 類型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| notificationId | string | path | 是 |  |
| newStatus | string | path | 是 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateUserNotificationStatus200Response.php)

## 範例

[inline-code-attrs-start title = 'updateUserNotificationStatus 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作了 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字串
$notification_id = 'notification_id_example'; // 字串
$new_status = 'new_status_example'; // 字串
$sso = 'sso_example'; // 字串

try {
    $result = $apiInstance->updateUserNotificationStatus($tenant_id, $notification_id, $new_status, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->updateUserNotificationStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---