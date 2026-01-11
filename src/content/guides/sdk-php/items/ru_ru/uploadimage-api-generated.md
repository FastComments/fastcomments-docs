Загрузить и изменить размер изображения

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Набор размеров: "Default" (1000x1000px) или "CrossPlatform" (создает размеры для популярных устройств) |
| urlId | string | query | No | Идентификатор страницы, с которой выполняется загрузка, для настройки |

## Ответ

Возвращает: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Пример

[inline-code-attrs-start title = 'Пример uploadImage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Если вы хотите использовать собственный HTTP-клиент, передайте ваш клиент, который реализует `GuzzleHttp\ClientInterface`.
    // Это необязательно, по умолчанию будет использован `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | Набор размеров: \"Default\" (1000x1000px) или \"CrossPlatform\" (создает размеры для популярных устройств)
$url_id = 'url_id_example'; // string | Идентификатор страницы, с которой выполняется загрузка, для настройки

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]