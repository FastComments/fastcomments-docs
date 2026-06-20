## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| commentId | string | path | Sì |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentTextResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio di getModerationCommentText'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se desideri utilizzare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale: verrà usato `GuzzleHttp\Client` come impostazione predefinita.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // stringa
$sso = 'sso_example'; // stringa

try {
    $result = $apiInstance->getModerationCommentText($comment_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getModerationCommentText: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]