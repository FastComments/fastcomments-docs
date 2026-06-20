## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserInternalProfileResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getUserInternalProfile Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du give din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit, `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getUserInternalProfile($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getUserInternalProfile: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]