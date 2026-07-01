## Parameters

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|--------------|-------------|
| tenantId | string | query | Oui |  |
| value | string | query | Non |  |
| sso | string | query | Non |  |

## Response

Retourne : [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationPageSearchResponse.php)

## Exemple

[inline-code-attrs-start title = 'getSearchPages Exemple'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // Si vous souhaitez utiliser un client http personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'value' => 'value_example', // string
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getSearchPages($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchPages: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]