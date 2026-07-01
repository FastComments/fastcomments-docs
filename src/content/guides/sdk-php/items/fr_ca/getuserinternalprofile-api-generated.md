## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|------------|------------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Renvoie : [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetUserInternalProfileResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple getUserInternalProfile'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'comment_id' => 'comment_id_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getUserInternalProfile($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getUserInternalProfile: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---