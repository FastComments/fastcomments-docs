Upload and resize an image
============================

## Parameters

| Nom   | Type   | Emplacement | Obligatoire | Description |
|-------|--------|-------------|-------------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Jeu de tailles : "Default" (1000x1000px) ou "CrossPlatform" (crée des tailles pour les appareils populaires) |
| urlId | string | query | No | ID de page depuis lequel le téléversement a lieu, à configurer |

## Réponse

Retourne : [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Exemple

[inline-code-attrs-start title = 'Exemple uploadImage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Si vous souhaitez utiliser un client HTTP personnalisé, passez votre client qui implémente `GuzzleHttp\ClientInterface`.
    // Ceci est optionnel, `GuzzleHttp\Client` sera utilisé par défaut.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$options = [
    'size_preset' => new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(), // \FastComments\Client\Model\SizePreset | Jeu de tailles : \"Default\" (1000x1000px) ou \"CrossPlatform\" (crée des tailles pour les appareils populaires)
    'url_id' => 'url_id_example', // string | ID de page depuis lequel le téléversement a lieu, à configurer
];


try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]