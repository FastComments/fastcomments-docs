## 參數

| 名稱 | 類型 | 位置 | 必要 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## 回應

返回：[`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentsResponse.php)

## 範例

[inline-code-attrs-start title = 'getComments 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// 設定 API 金鑰授權：api_key
$config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKey('x-api-key', 'YOUR_API_KEY');
// 取消註解以下以設定 API 金鑰的前綴（例如 Bearer），如有需要
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // 如果您想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，`GuzzleHttp\Client` 會被用作預設。
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // 字串
$options = [
    'page' => 56, // 整數
    'limit' => 56, // 整數
    'skip' => 56, // 整數
    'as_tree' => True, // 布林
    'skip_children' => 56, // 整數
    'limit_children' => 56, // 整數
    'max_tree_depth' => 56, // 整數
    'url_id' => 'url_id_example', // 字串
    'user_id' => 'user_id_example', // 字串
    'anon_user_id' => 'anon_user_id_example', // 字串
    'context_user_id' => 'context_user_id_example', // 字串
    'hash_tag' => 'hash_tag_example', // 字串
    'parent_id' => 'parent_id_example', // 字串
    'direction' => new \FastComments\Client\Model\\FastComments\Client\Model\SortDirections(), // \FastComments\Client\Model\SortDirections
    'from_date' => 56, // 整數
    'to_date' => 56, // 整數
];


try {
    $result = $apiInstance->getComments($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComments: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]