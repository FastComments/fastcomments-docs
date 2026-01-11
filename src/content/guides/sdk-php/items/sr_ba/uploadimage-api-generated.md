Otpremi i promijeni veličinu slike

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| sizePreset | string | query | Ne | Podešavanje veličine: "Default" (1000x1000px) ili "CrossPlatform" (kreira veličine za popularne uređaje) |
| urlId | string | query | Ne | ID stranice sa koje se vrši upload, za konfiguraciju |

## Odgovor

Vraća: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Primjer

[inline-code-attrs-start title = 'Primjer za uploadImage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ako želite koristiti prilagođeni HTTP klijent, proslijedite vaš klijent koji implementira `GuzzleHttp\ClientInterface`.
    // Ovo je opcionalno, `GuzzleHttp\Client` će biti korišten kao zadani.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | Podešavanje veličine: \"Default\" (1000x1000px) ili \"CrossPlatform\" (pravljenje veličina za popularne uređaje)
$url_id = 'url_id_example'; // string | ID stranice sa koje se vrši upload, za konfiguraciju

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]