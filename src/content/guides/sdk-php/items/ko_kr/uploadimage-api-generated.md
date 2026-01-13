이미지 업로드 및 크기 조정

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | 사이즈 프리셋: "Default" (1000x1000px) 또는 "CrossPlatform" (일반적인 기기에 맞는 크기를 생성) |
| urlId | string | query | No | 업로드가 발생하는 페이지의 id(구성용) |

## 응답

반환: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## 예제

[inline-code-attrs-start title = 'uploadImage 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현한 클라이언트를 전달하세요.
    // 선택 사항입니다. 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$size_preset = new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(); // \FastComments\Client\Model\SizePreset | 사이즈 프리셋: \"Default\" (1000x1000px) 또는 \"CrossPlatform\" (일반적인 기기에 맞는 크기를 생성)
$url_id = 'url_id_example'; // string | 업로드가 발생하는 페이지의 id, 구성용

try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $size_preset, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]