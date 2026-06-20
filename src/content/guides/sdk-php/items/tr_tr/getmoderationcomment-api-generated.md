## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| includeEmail | boolean | query | Hayır |  |
| includeIP | boolean | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICommentResponse.php)

## Örnek

[inline-code-attrs-start title = 'getModerationComment Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface`'i uygulayan istemcinizi iletin.
    // Bu isteğe bağlıdır, `GuzzleHttp\Client` varsayılan olarak kullanılacaktır.
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

---