## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Nee |  |
| commentId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserManualBadgesResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getManualBadgesForUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Als u een aangepaste http-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);
$badges_user_id = 'badges_user_id_example'; // string
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getManualBadgesForUser($badges_user_id, $comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getManualBadgesForUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]