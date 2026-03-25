## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | はい |  |
| id | string | path | はい |  |

## レスポンス

戻り値: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ChangeTicketState200Response.php)

## 例

[inline-code-attrs-start title = 'changeTicketStateの例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 必要に応じて、APIキーのプレフィックス（例: Bearer）を設定するには、下の行のコメントを外してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // カスタムのHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface`を実装するクライアントを渡してください。
    // これはオプションです。デフォルトでは`GuzzleHttp\Client`が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$user_id = 'user_id_example'; // string
$id = 'id_example'; // string
$change_ticket_state_body = new \FastComments\Client\Model\ChangeTicketStateBody(); // \FastComments\Client\Model\ChangeTicketStateBody

try {
    $result = $apiInstance->changeTicketState($tenant_id, $user_id, $id, $change_ticket_state_body);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->changeTicketState: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---