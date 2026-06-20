## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| includeEmail | boolean | query | 否 |  |
| includeIP | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回：[`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICommentResponse.php)

## 示例

[inline-code-attrs-start title = 'getModerationComment 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自定义 HTTP 客户端，请传入实现了 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$include_email = True; // bool
$include_ip = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getModerationComment($comment_id, $include_email, $include_ip, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getModerationComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]