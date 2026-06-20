## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | query | 否 |  |

## 响应

返回: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPIChildCommentsResponse.php)

## 示例

[inline-code-attrs-start title = 'postCommentsByIds 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$comments_by_ids_params = new \FastComments\Client\Model\CommentsByIdsParams(); // \FastComments\Client\Model\CommentsByIdsParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postCommentsByIds($comments_by_ids_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postCommentsByIds: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]