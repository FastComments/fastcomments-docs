## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| urlId | string | クエリ | いいえ |  |
| userId | string | クエリ | いいえ |  |
| startDate | string | クエリ | いいえ |  |
| questionId | string | クエリ | いいえ |  |
| questionIds | string | クエリ | いいえ |  |
| skip | number | クエリ | いいえ |  |

## レスポンス

返却値: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetQuestionResults200Response.php)

## 例

[inline-code-attrs-start title = 'getQuestionResults の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// APIキー認証を設定: api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// 必要な場合は以下のコメントを外して API キーのプレフィックス（例: Bearer）を設定してください
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // カスタムHTTPクライアントを使用する場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // これはオプションで、デフォルトでは `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client(),
    $config
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$user_id = 'user_id_example'; // string
$start_date = 'start_date_example'; // string
$question_id = 'question_id_example'; // string
$question_ids = 'question_ids_example'; // string
$skip = 3.4; // float

try {
    $result = $apiInstance->getQuestionResults($tenant_id, $url_id, $user_id, $start_date, $question_id, $question_ids, $skip);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getQuestionResults: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]