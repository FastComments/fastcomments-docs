## 參數

| 名稱 | Type | Location | Required | 說明 |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| sso | string | query | No |  |

## 回應

回傳: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationSuggestResponse.php)

## 範例

[inline-code-attrs-start title = 'getSearchSuggest 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 若要使用自訂的 HTTP 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 此為選用，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // 字串
$sso = 'sso_example'; // 字串

try {
    $result = $apiInstance->getSearchSuggest($text_search, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchSuggest: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---