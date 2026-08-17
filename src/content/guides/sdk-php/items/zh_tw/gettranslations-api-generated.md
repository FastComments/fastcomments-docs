## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| namespace | 字串 | 路徑 | 是 |  |
| component | 字串 | 路徑 | 是 |  |
| locale | 字串 | 查詢 | 否 |  |
| useFullTranslationIds | 布林 | 查詢 | 否 |  |

## 回應

返回：[`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTranslationsResponse.php)

## 範例

[inline-code-attrs-start title = 'getTranslations 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$namespace = 'namespace_example'; // 字串
$component = 'component_example'; // 字串
$options = [
    'locale' => 'locale_example', // 字串
    'use_full_translation_ids' => True, // 布林
];


try {
    $result = $apiInstance->getTranslations($namespace, $component, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getTranslations: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]