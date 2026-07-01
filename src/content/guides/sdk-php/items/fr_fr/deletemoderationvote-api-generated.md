## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|--------------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |
| voteId | string | path | Oui |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/VoteDeleteResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple deleteModerationVote'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est facultatif, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$comment_id = 'comment_id_example'; // string
$vote_id = 'vote_id_example'; // string
$options = [
    'broadcast_id' => 'broadcast_id_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->deleteModerationVote($tenant_id, $comment_id, $vote_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->deleteModerationVote: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]