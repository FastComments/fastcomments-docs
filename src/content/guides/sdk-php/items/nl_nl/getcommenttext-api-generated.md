## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| editKey | string | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Retourneert: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentText200Response.php)

## Voorbeeld

[inline-code-attrs-start title = 'getCommentText Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef dan uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$edit_key = 'edit_key_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCommentText($tenant_id, $comment_id, $edit_key, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentText: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]