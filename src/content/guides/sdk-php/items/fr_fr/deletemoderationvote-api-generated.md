## Paramètres

| Nom | Type | Location | Obligatoire | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| voteId | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteDeleteResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de deleteModerationVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // chaîne
$vote_id = 'vote_id_example'; // chaîne
$sso = 'sso_example'; // chaîne

try {
    $result = $apiInstance->deleteModerationVote($comment_id, $vote_id, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->deleteModerationVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]