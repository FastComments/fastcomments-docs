## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| sso | string | requête | No |  |

## Réponse

Renvoie: [`GetTenantManualBadgesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTenantManualBadgesResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple de getManualBadges'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel : `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getManualBadges($sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getManualBadges: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]