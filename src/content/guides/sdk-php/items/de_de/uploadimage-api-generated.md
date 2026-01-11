Bild hochladen und skalieren

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nein | Größen-Preset: "Default" (1000x1000px) oder "CrossPlatform" (erstellt Größen für gängige Geräte) |
| urlId | string | query | Nein | Seiten-ID, von der der Upload erfolgt, zur Konfiguration |

## Antwort

Gibt zurück: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Beispiel

[inline-code-attrs-start title = 'uploadImage Beispiel'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Wenn Sie einen benutzerdefinierten HTTP-Client verwenden möchten, übergeben Sie Ihren Client, der `GuzzleHttp\ClientInterface` implementiert.
    // Dies ist optional, `GuzzleHttp\Client` wird standardmäßig verwendet.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | Größen-Preset: \"Default\" (1000x1000px) oder \"CrossPlatform\" (erstellt Größen für gängige Geräte)
$url_id = 'url_id_example'; // string | Seiten-ID, von der der Upload erfolgt, zur Konfiguration

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---