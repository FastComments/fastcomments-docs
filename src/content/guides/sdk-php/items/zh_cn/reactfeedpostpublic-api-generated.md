## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| postId | string | 路径 | 是 |  |
| isUndo | boolean | 查询 | 否 |  |
| broadcastId | string | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ReactFeedPostPublic200Response.php)

## 示例

[inline-code-attrs-start title = 'reactFeedPostPublic 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义 HTTP 客户端，传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认会使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字符串
$post_id = 'post_id_example'; // 字符串
$react_body_params = new \FastComments\Client\Model\ReactBodyParams(); // \FastComments\Client\Model\ReactBodyParams
$is_undo = True; // 布尔值
$broadcast_id = 'broadcast_id_example'; // 字符串
$sso = 'sso_example'; // 字符串

try {
    $result = $apiInstance->reactFeedPostPublic($tenant_id, $post_id, $react_body_params, $is_undo, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->reactFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---