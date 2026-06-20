## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`AdjustVotesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/AdjustVotesResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de postAdjustCommentVotes'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, fournissez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$adjust_comment_votes_params = new \FastComments\Client\Model\AdjustCommentVotesParams(); // \FastComments\Client\Model\AdjustCommentVotesParams
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->postAdjustCommentVotes($comment_id, $adjust_comment_votes_params, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postAdjustCommentVotes: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]