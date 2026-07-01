Upload and resize an image
============================

## Parameters

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Профіль розміру: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв) |
| urlId | string | query | No | ID сторінки, з якої відбувається завантаження, для налаштування |

## Response

Повертає: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Example

[inline-code-attrs-start title = 'uploadImage приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP клієнт, передайте ваш клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, `GuzzleHttp\Client` буде використаний за замовчуванням.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // рядок
$file = '/path/to/file.txt'; // \SplFileObject
$options = [
    'size_preset' => new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(), // \FastComments\Client\Model\SizePreset | Профіль розміру: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв)
    'url_id' => 'url_id_example', // рядок | ID сторінки, з якої відбувається завантаження, для налаштування
];


try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]