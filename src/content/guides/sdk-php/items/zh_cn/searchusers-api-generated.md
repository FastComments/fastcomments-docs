## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| usernameStartsWith | string | query | 否 |  |
| mentionGroupIds | array | query | 否 |  |
| sso | string | query | 否 |  |
| searchSection | string | query | 否 |  |

## 响应

返回: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SearchUsersResult.php)

## 示例

[inline-code-attrs-start title = 'searchUsers 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义的 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字符串
$url_id = 'url_id_example'; // 字符串
$username_starts_with = 'username_starts_with_example'; // 字符串
$mention_group_ids = array('mention_group_ids_example'); // 字符串[]
$sso = 'sso_example'; // 字符串
$search_section = 'search_section_example'; // 字符串

try {
    $result = $apiInstance->searchUsers($tenant_id, $url_id, $username_starts_with, $mention_group_ids, $sso, $search_section);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->searchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]