Upload and resize an image
==========================

## Parameters

| Назва | Тип | Розташування | Обов’язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| sizePreset | string | query | Ні | Попередньо визначений розмір: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв) |
| urlId | string | query | Ні | Ідентифікатор сторінки, з якої виконується завантаження, для налаштування |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Example

[inline-code-attrs-start title = 'uploadImage Приклад'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви хочете використовувати власний HTTP-клієнт, передайте свій клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов’язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$options = [
    'size_preset' => new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(), // \FastComments\Client\Model\SizePreset | Попередньо визначений розмір: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв)
    'url_id' => 'url_id_example', // string | Ідентифікатор сторінки, з якої виконується завантаження, для налаштування
];


try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]