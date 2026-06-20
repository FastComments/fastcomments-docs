## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`VoteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di postVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se vuoi usare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` come predefinito.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$direction = 'direction_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postVote($comment_id, $direction, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]