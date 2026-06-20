## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | sti | Ja |  |
| reviewed | boolean | forespørgsel | Nej |  |
| sso | string | forespørgsel | Nej |  |

## Svar

Returnerer: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Eksempel

[inline-code-attrs-start title = 'postSetCommentReviewStatus Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du videregive din klient, som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` bruges som standard.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$reviewed = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postSetCommentReviewStatus($comment_id, $reviewed, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentReviewStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]