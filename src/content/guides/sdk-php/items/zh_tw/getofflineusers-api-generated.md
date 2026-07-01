過去在此頁面上發表過評論，但目前不在線上的使用者。依 displayName 排序。  
在使用完 /users/online 之後，使用此端點來呈現「Members」區段。  
使用 commenterName 的游標分頁：伺服器從 afterName 開始，透過 $gt 向前遍歷部分 {tenantId, urlId, commenterName} 索引，無需 $skip 成本。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別碼（在伺服器端已清理）。 |
| afterName | string | query | No | 游標：傳入前一次回應中的 nextAfterName。 |
| afterUserId | string | query | No | 游標平手時的分割依據：傳入前一次回應中的 nextAfterUserId。當設定 afterName 時必填，以避免名稱相同的項目被遺失。 |

## 回應

Returns: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## 示例

[inline-code-attrs-start title = '取得離線使用者 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 如果您想使用自訂的 HTTP 客戶端，傳入實作 `GuzzleHttp\ClientInterface` 的客戶端。
    // 這是可選的，將預設使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | 頁面 URL 識別碼（在伺服器端已清理）。
$options = [
    'after_name' => 'after_name_example', // string | 游標：傳入前一次回應中的 nextAfterName。
    'after_user_id' => 'after_user_id_example', // string | 游標平手時的分割依據：傳入前一次回應中的 nextAfterUserId。當設定 afterName 時必填，以避免名稱相同的項目被遺失。
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---