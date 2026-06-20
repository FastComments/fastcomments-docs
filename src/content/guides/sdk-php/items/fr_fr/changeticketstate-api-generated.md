## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Oui |  |
| id | string | path | Oui |  |

## Réponse

Renvoie: [`ChangeTicketStateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ChangeTicketStateResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de changeTicketState'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurer l'autorisation par clé API : api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Décommentez ci-dessous pour configurer le préfixe (ex. Bearer) pour la clé API, si nécessaire
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous voulez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$id = 'id_example'; // string
$change_ticket_state_body = new \FastComments\Client\Model\ChangeTicketStateBody(); // \FastComments\Client\Model\ChangeTicketStateBody

try {
    $result = $apiInstance->changeTicketState($tenant_id, $user_id, $id, $change_ticket_state_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->changeTicketState: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]