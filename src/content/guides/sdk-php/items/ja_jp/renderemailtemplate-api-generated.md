---
## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| locale | string | query | いいえ |  |

## レスポンス

戻り値: [`RenderEmailTemplateResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/RenderEmailTemplateResponse.php)

## 例

[inline-code-attrs-start title = 'renderEmailTemplate の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証の設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、下記のコメントを解除して API キーのプレフィックス（例: Bearer）を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムの HTTP クライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$render_email_template_body = new \FastComments\Client\Model\RenderEmailTemplateBody(); // \FastComments\Client\Model\RenderEmailTemplateBody
$locale = 'locale_example'; // string

try {
    $result = $apiInstance->renderEmailTemplate($tenant_id, $render_email_template_body, $locale);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->renderEmailTemplate: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---