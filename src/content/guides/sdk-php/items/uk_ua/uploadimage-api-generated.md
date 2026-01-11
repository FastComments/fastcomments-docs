Завантажити та змінити розмір зображення

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| sizePreset | string | query | Ні | Параметр розміру: "Default" (1000x1000px) або "CrossPlatform" (створює розміри для популярних пристроїв) |
| urlId | string | query | Ні | ID сторінки, з якої виконується завантаження, для налаштування |

## Відповідь

Повертає: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Приклад

[inline-code-attrs-start title = 'Приклад uploadImage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Якщо ви бажаєте використати користувацький HTTP-клієнт, передайте клієнт, який реалізує `GuzzleHttp\ClientInterface`.
    // Це необов'язково, за замовчуванням буде використано `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | Параметр розміру: \"Default\" (1000x1000px) або \"CrossPlatform\" (створює розміри для популярних пристроїв)
$url_id = 'url_id_example'; // string | ID сторінки, з якої виконується завантаження, для налаштування

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]