## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| commentId | string | path | Evet |  |
| dir | integer | query | Evet |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentVoteUserNames200Response.php)

## Örnek

[inline-code-attrs-start title = 'getCommentVoteUserNames Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Eğer özel bir http istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$dir = 56; // int
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCommentVoteUserNames($tenant_id, $comment_id, $dir, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentVoteUserNames: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]