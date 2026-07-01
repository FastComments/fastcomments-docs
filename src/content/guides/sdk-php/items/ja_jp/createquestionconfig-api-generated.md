## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

返却: [`CreateQuestionConfigResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateQuestionConfigResponse.php)

## 例

[inline-code-attrs-start title = 'createQuestionConfig 例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、以下のコメントアウトを解除してプレフィックス (例: Bearer) を設定します
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$create_question_config_body = new \FastComments\Client\Model\CreateQuestionConfigBody(); // \FastComments\Client\Model\CreateQuestionConfigBody


try {
    $result = $apiInstance->createQuestionConfig($tenant_id, $create_question_config_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createQuestionConfig: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]