Upload og tilpas størrelse på et billede

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nej | Størrelsesforudindstilling: "Default" (1000x1000px) eller "CrossPlatform" (opretter størrelser til populære enheder) |
| urlId | string | query | Nej | Side-id hvorfra uploaden foretages, til konfiguration |

## Svar

Returnerer: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Eksempel

[inline-code-attrs-start title = 'uploadImage Eksempel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Hvis du vil bruge en brugerdefineret HTTP-klient, skal du give din klient som implementerer `GuzzleHttp\ClientInterface`.
    // Dette er valgfrit; `GuzzleHttp\Client` vil blive brugt som standard.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | Størrelsesforudindstilling: \"Default\" (1000x1000px) eller \"CrossPlatform\" (opretter størrelser til populære enheder)
$url_id = 'url_id_example'; // string | Side-id hvorfra uploaden foretages, til konfiguration

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]