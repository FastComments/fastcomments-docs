Upload and resize an image → 이미지 업로드 및 크기 조정

## Parameters

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| sizePreset | string | query | 아니오 | Size preset: "Default" (1000x1000px) 또는 "CrossPlatform" (인기 디바이스용 크기를 생성) |
| urlId | string | query | 아니오 | Page id that upload is happening from, to configure → 업로드가 발생하는 페이지 ID, 구성용 |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php) → 반환: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Example

[inline-code-attrs-start title = 'uploadImage 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하십시오.
    // 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$options = [
    'size_preset' => new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(), // \FastComments\Client\Model\SizePreset | Size preset: "Default" (1000x1000px) 또는 "CrossPlatform" (인기 디바이스용 크기를 생성)
    'url_id' => 'url_id_example', // string | Page id that upload is happening from, to configure → 업로드가 발생하는 페이지 ID, 구성용
];


try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]