まず[installation procedure](#installation-usage-readme-generated)に従ってください。その後、以下を実行してください:

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



// Configure API key authorization: api_key
// APIキー認証を設定: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには下の行のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // カスタムHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface`を実装するクライアントを渡してください。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // これは任意です。デフォルトでは`GuzzleHttp\Client`が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$add_domain_config_params = new \FastComments\Client\Model\AddDomainConfigParams(); // \FastComments\Client\Model\AddDomainConfigParams

try {
    $result = $apiInstance->addDomainConfig($tenant_id, $add_domain_config_params);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->addDomainConfig: ', $e->getMessage(), PHP_EOL;
}

```