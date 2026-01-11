Отпреми и промени величину слике

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Пресет величине: "Default" (1000x1000px) или "CrossPlatform" (генерише величине за популарне уређаје) |
| urlId | string | query | No | Ид странице са које се врши отпремање, за конфигурацију |

## Одговор

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример uploadImage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Ако желите да користите прилагођени HTTP клијент, проследите ваш клијент који имплементира `GuzzleHttp\ClientInterface`.
    // Ово је опционално, подразумевани ће бити `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | Пресет величине: \"Default\" (1000x1000px) или \"CrossPlatform\" (генерише величине за популарне уређаје)
$url_id = 'url_id_example'; // string | Ид странице са које се врши отпремање, за конфигурацију

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]