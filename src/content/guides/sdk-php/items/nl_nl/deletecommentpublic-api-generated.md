## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| broadcastId | string | query | Ja |  |
| editKey | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`DeleteCommentPublic200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/DeleteCommentPublic200Response.php)

## Voorbeeld

[inline-code-attrs-start title = 'deleteCommentPublic Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client mee die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` zal standaard worden gebruikt.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$broadcast_id = 'broadcast_id_example'; // string
$edit_key = 'edit_key_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->deleteCommentPublic($tenant_id, $comment_id, $broadcast_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteCommentPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]