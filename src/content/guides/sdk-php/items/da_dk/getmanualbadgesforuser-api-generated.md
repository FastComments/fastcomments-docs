## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | Nej |  |
| commentId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserManualBadgesResponse.php)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getManualBadgesForUser'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, videregiv din klient som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
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