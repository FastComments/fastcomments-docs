## 參數

| 名稱 | 型別 | 位置 | 必要 | 說明 |
|------|------|----------|----------|-------------|
| commentId | string | path | 是 |  |
| spam | boolean | query | 否 |  |
| permNotSpam | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## 範例

[inline-code-attrs-start title = 'postSetCommentSpamStatus 範例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 如果您想使用自訂的 http 用戶端，請傳入實作 `GuzzleHttp\ClientInterface` 的用戶端。
    // 這是可選的，預設會使用 `GuzzleHttp\Client`。
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // 字串
$spam = True; // 布林值
$perm_not_spam = True; // 布林值
$sso = 'sso_example'; // 字串

try {
    $result = $apiInstance->postSetCommentSpamStatus($comment_id, $spam, $perm_not_spam, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postSetCommentSpamStatus: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]