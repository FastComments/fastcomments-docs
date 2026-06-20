## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`PostRemoveCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PostRemoveCommentResponse.php)

## Örnek

[inline-code-attrs-start title = 'postRemoveComment Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postRemoveComment($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postRemoveComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]