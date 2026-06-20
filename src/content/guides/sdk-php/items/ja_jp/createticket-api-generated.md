## パラメーター

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | はい |  |

## レスポンス

戻り値: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/CreateTicketResponse.php)

## 例

[inline-code-attrs-start title = 'createTicket の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証の設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには下の行のコメントを解除してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムHTTPクライアントを使用する場合、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これは任意です。デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$create_ticket_body = new \FastComments\Client\Model\CreateTicketBody(); // \FastComments\Client\Model\CreateTicketBody

try {
    $result = $apiInstance->createTicket($tenant_id, $user_id, $create_ticket_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->createTicket: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]