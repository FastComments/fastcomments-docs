## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| urlId | string | 查询 | 是 |  |
| id | string | 查询 | 是 |  |
| title | string | 查询 | 否 |  |

## 响应

返回: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateV1PageReact.php)

## 示例

[inline-code-attrs-start title = 'createV2PageReact 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义的 HTTP 客户端，请传入实现 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字符串
$url_id = 'url_id_example'; // 字符串
$id = 'id_example'; // 字符串
$title = 'title_example'; // 字符串

try {
    $result = $apiInstance->createV2PageReact($tenant_id, $url_id, $id, $title);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createV2PageReact: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]