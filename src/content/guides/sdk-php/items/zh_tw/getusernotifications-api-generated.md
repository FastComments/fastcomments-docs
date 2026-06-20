## 參數

| 名稱 | 類型 | 位置 | 必須 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| urlId | string | query | 否 | 用於判斷目前頁面是否已訂閱。 |
| pageSize | integer | query | 否 |  |
| afterId | string | query | 否 |  |
| includeContext | boolean | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| includeTranslations | boolean | query | 否 |  |
| includeTenantNotifications | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetMyNotificationsResponse.php)

## 範例

[inline-code-attrs-start title = 'getUserNotifications 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 用於判斷目前頁面是否已訂閱。
$page_size = 56; // int
$after_id = 'after_id_example'; // string
$include_context = True; // bool
$after_created_at = 56; // int
$unread_only = True; // bool
$dm_only = True; // bool
$no_dm = True; // bool
$include_translations = True; // bool
$include_tenant_notifications = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserNotifications($tenant_id, $url_id, $page_size, $after_id, $include_context, $after_created_at, $unread_only, $dm_only, $no_dm, $include_translations, $include_tenant_notifications, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]