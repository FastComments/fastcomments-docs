## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| spam | boolean | query | No |  |
| permNotSpam | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Esempio

[inline-code-attrs-start title = 'Esempio postSetCommentSpamStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Se vuoi utilizzare un client HTTP personalizzato, passa il tuo client che implementa `GuzzleHttp\ClientInterface`.
    // Questo è opzionale, verrà usato `GuzzleHttp\Client` come predefinito.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // stringa
$comment_id = 'comment_id_example'; // stringa
$options = [
    'spam' => True, // bool
    'perm_not_spam' => True, // bool
    'broadcast_id' => 'broadcast_id_example', // stringa
    'sso' => 'sso_example', // stringa
];


try {
    $result = $apiInstance->postSetCommentSpamStatus($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentSpamStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]