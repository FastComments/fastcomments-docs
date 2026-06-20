## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| namespace | string | path | 是 |  |
| component | string | path | 是 |  |
| locale | string | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |

## 响应

返回: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTranslationsResponse.php)

## 示例

[inline-code-attrs-start title = 'getTranslations 示例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自定义的 HTTP 客户端，请传入实现 `GuzzleHttp\ClientInterface` 的客户端。
    // 这是可选的，默认将使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$namespace = 'namespace_example'; // 字符串
$component = 'component_example'; // 字符串
$locale = 'locale_example'; // 字符串
$use_full_translation_ids = True; // 布尔值

try {
    $result = $apiInstance->getTranslations($namespace, $component, $locale, $use_full_translation_ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getTranslations: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---