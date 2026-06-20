## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| afterId | string | query | 否 |  |
| afterCreatedAt | integer | query | 否 |  |
| unreadOnly | boolean | query | 否 |  |
| dmOnly | boolean | query | 否 |  |
| noDm | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`ResetUserNotificationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ResetUserNotificationsResponse.php)

## 示例

[inline-code-attrs-start title = 'resetUserNotifications 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$after_id = 'after_id_example'; // string
$after_created_at = 56; // int
$unread_only = True; // bool
$dm_only = True; // bool
$no_dm = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->resetUserNotifications($tenant_id, $after_id, $after_created_at, $unread_only, $dm_only, $no_dm, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->resetUserNotifications: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]