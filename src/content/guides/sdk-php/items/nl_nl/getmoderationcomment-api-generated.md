## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| includeEmail | boolean | query | Nee |  |
| includeIP | boolean | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICommentResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getModerationComment Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
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