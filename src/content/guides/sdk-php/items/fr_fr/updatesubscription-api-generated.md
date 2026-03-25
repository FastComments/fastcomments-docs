## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| userId | string | query | Non |  |

## Réponse

Renvoie: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UpdateSubscriptionAPIResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple pour updateSubscription'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurer l'autorisation par clé API : api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Décommentez ci-dessous pour configurer le préfixe (p.ex. Bearer) pour la clé API, si nécessaire
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, fournissez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$update_api_user_subscription_data = new \FastComments\Client\Model\UpdateAPIUserSubscriptionData(); // \FastComments\Client\Model\UpdateAPIUserSubscriptionData
$user_id = 'user_id_example'; // string

try {
    $result = $apiInstance->updateSubscription($tenant_id, $id, $update_api_user_subscription_data, $user_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->updateSubscription: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]