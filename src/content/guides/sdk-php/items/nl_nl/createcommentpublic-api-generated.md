## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | pad | Ja |  |
| urlId | string | query | Ja |  |
| broadcastId | string | query | Ja |  |
| sessionId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`CreateCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateCommentPublic200Response.php)

## Voorbeeld

[inline-code-attrs-start title = 'createCommentPublic Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$comment_data = new \FastComments\Client\Model\CommentData(); // \FastComments\Client\Model\CommentData
$session_id = 'session_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->createCommentPublic($tenant_id, $url_id, $broadcast_id, $comment_data, $session_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->createCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]