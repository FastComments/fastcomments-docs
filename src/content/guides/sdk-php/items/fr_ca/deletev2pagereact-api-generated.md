## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|--------------|-------------|
| tenantId | string | chemin | Oui |  |
| urlId | string | requête | Oui |  |
| id | string | requête | Oui |  |

## Réponse

Renvoie : [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateV1PageReact.php)

## Exemple

[inline-code-attrs-start title = 'deleteV2PageReact Exemple'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, transmettez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$id = 'id_example'; // string


try {
    $result = $apiInstance->deleteV2PageReact($tenant_id, $url_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->deleteV2PageReact: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]