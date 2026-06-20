## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | sti | Ja |  |
| sso | string | forespørgsel | Nej |  |

## Svar

Returnerer: [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentBanStatusResponse.php)

## Eksempel

[inline-code-attrs-start title = 'getCommentBanStatus-eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du give din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getCommentBanStatus($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCommentBanStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]