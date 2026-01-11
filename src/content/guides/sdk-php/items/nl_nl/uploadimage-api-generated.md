Een afbeelding uploaden en schalen

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nee | Groottevoorinstelling: "Default" (1000x1000px) of "CrossPlatform" (maakt formaten voor populaire apparaten) |
| urlId | string | query | Nee | Pagina-id waarvan de upload afkomstig is, om te configureren |

## Response

Retourneert: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Example

[inline-code-attrs-start title = 'uploadImage Voorbeeld'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Als u een aangepaste HTTP-client wilt gebruiken, geef uw client door die `GuzzleHttp\ClientInterface` implementeert.
    // Dit is optioneel, `GuzzleHttp\Client` wordt standaard gebruikt.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | Groottevoorinstelling: \"Default\" (1000x1000px) of \"CrossPlatform\" (maakt formaten voor populaire apparaten)
$url_id = 'url_id_example'; // string | Pagina-id waarvan de upload afkomstig is, om te configureren

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---