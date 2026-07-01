## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |

## Response

Renvoie : [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple replaceTenantPackage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configurer l'autorisation de la clé API : api_key
// Décommentez ci‑dessous pour configurer le préfixe (par ex. Bearer) pour la clé API, si nécessaire
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // chaîne
$id = 'id_example'; // chaîne
$replace_tenant_package_body = new \FastComments\Client\Model\ReplaceTenantPackageBody(); // \FastComments\Client\Model\ReplaceTenantPackageBody


try {
    $result = $apiInstance->replaceTenantPackage($tenant_id, $id, $replace_tenant_package_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->replaceTenantPackage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]