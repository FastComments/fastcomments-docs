## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Yes |  |
| isLive | boolean | query | No |  |
| doSpamCheck | boolean | query | No |  |
| sendEmails | boolean | query | No |  |
| populateNotifications | boolean | query | No |  |

## Réponse

Retourne : [`SaveCommentsBulkResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SaveCommentsBulkResponse.php)

## Exemple

[inline-code-attrs-start title = 'saveCommentsBulk Exemple'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurer l'autorisation de clé API : api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Décommentez ci‑dessus pour configurer le préfixe (p. ex. Bearer) pour la clé API, si nécessaire
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous voulez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // chaîne
$create_comment_params = array(new \FastComments\Client\Model\CreateCommentParams()); // \FastComments\Client\Model\CreateCommentParams[]
$options = [
    'is_live' => True, // booléen
    'do_spam_check' => True, // booléen
    'send_emails' => True, // booléen
    'populate_notifications' => True, // booléen
];


try {
    $result = $apiInstance->saveCommentsBulk($tenant_id, $create_comment_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->saveCommentsBulk: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---