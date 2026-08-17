## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| namespace | string | path | はい |  |
| component | string | path | はい |  |
| locale | string | query | いいえ |  |
| useFullTranslationIds | boolean | query | いいえ |  |

## レスポンス

返却: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetTranslationsResponse.php)

## 例

[inline-code-attrs-start title = 'getTranslations 例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションで、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$namespace = 'namespace_example'; // 文字列
$component = 'component_example'; // 文字列
$options = [
    'locale' => 'locale_example', // 文字列
    'use_full_translation_ids' => True, // ブール
];


try {
    $result = $apiInstance->getTranslations($namespace, $component, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getTranslations: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---