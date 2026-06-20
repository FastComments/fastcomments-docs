## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| reviewed | boolean | query | No |  |
| sso | string | query | No |  |

## Odpowiedź

Zwraca: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład postSetCommentReviewStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż swój klient implementujący `GuzzleHttp\ClientInterface`.
    // To opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
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