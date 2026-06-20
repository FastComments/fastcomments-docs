## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetCommentBanStatusResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetCommentBanStatusResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentBanStatus'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
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