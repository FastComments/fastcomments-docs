## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | 예 |  |
| includeByUserIdAndEmail | boolean | query | 아니요 |  |
| includeByIP | boolean | query | 아니요 |  |
| includeByEmailDomain | boolean | query | 아니요 |  |
| sso | string | query | 아니요 |  |

## 응답

반환: [`PreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PreBanSummary.php)

## 예제

[inline-code-attrs-start title = 'getPreBanSummary 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 맞춤 HTTP 클라이언트를 사용하려면, `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항이며 기본으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$comment_id = 'comment_id_example'; // string
$include_by_user_id_and_email = True; // bool
$include_by_ip = True; // bool
$include_by_email_domain = True; // bool
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getPreBanSummary($comment_id, $include_by_user_id_and_email, $include_by_ip, $include_by_email_domain, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---