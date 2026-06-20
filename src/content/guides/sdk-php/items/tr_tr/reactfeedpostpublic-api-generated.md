## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| postId | string | path | Evet |  |
| isUndo | boolean | query | Hayır |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Response

Döndürür: [`ReactFeedPostResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ReactFeedPostResponse.php)

## Örnek

[inline-code-attrs-start title = 'reactFeedPostPublic Örneği'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Özel bir HTTP istemcisi kullanmak istiyorsanız, `GuzzleHttp\ClientInterface` uygulayan istemcinizi geçin.
    // Bu isteğe bağlıdır, varsayılan olarak `GuzzleHttp\Client` kullanılacaktır.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$post_id = 'post_id_example'; // string
$react_body_params = new \FastComments\Client\Model\ReactBodyParams(); // \FastComments\Client\Model\ReactBodyParams
$is_undo = True; // bool
$broadcast_id = 'broadcast_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->reactFeedPostPublic($tenant_id, $post_id, $react_body_params, $is_undo, $broadcast_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->reactFeedPostPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]