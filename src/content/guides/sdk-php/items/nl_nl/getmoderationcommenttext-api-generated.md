## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| sso | string | query | Nee |  |

## Response

Retourneert: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentTextResponse.php)

## Voorbeeld

[inline-code-attrs-start title = 'getModerationCommentText Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geeft u uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel; `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getModerationCommentText($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getModerationCommentText: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---