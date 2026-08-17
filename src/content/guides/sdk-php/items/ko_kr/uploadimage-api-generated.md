Upload and resize an image

## Parameters

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| sizePreset | string | query | 아니오 | 크기 사전 설정: "Default" (1000x1000px) 또는 "CrossPlatform" (인기 기기용 크기 생성) |
| urlId | string | query | 아니오 | 업로드가 발생하는 페이지 ID, 구성용 |

## Response

반환: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Example

[inline-code-attrs-start title = 'uploadImage 예시'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 문자열
$file = '/path/to/file.txt'; // \SplFileObject
$options = [
    'size_preset' => new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(), // \FastComments\Client\Model\SizePreset | 크기 사전 설정: \"Default\" (1000x1000px) 또는 \"CrossPlatform\" (인기 기기용 크기 생성)
    'url_id' => 'url_id_example', // 문자열 | 업로드가 발생하는 페이지 ID, 구성용
];


try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]