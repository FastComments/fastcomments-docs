Prześlij i zmień rozmiar obrazu

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Tak |  |
| sizePreset | string | query | Nie | Predefiniowany rozmiar: "Default" (1000x1000px) lub "CrossPlatform" (tworzy rozmiary dla popularnych urządzeń) |
| urlId | string | query | Nie | Id strony, z której odbywa się przesyłanie, do konfiguracji |

## Odpowiedź

Zwraca: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia uploadImage'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // Jeśli chcesz użyć niestandardowego klienta HTTP, przekaż klienta implementującego `GuzzleHttp\ClientInterface`.
    // To jest opcjonalne, domyślnie zostanie użyty `GuzzleHttp\Client`.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | Predefiniowany rozmiar: \"Default\" (1000x1000px) lub \"CrossPlatform\" (tworzy rozmiary dla popularnych urządzeń)
$url_id = 'url_id_example'; // string | Id strony, z której odbywa się przesyłanie, do konfiguracji

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]