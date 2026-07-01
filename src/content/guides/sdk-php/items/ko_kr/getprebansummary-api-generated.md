## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| includeByUserIdAndEmail | boolean | query | No |  |
| includeByIP | boolean | query | No |  |
| includeByEmailDomain | boolean | query | No |  |
| sso | string | query | No |  |

## 응답

반환: [`PreBanSummary`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PreBanSummary.php)

## 예시

[inline-code-attrs-start title = 'getPreBanSummary 예시'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface` 를 구현하는 클라이언트를 전달하십시오.
    // 선택 사항이며, 기본값으로 `GuzzleHttp\Client` 가 사용됩니다.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 문자열
$comment_id = 'comment_id_example'; // 문자열
$options = [
    'include_by_user_id_and_email' => True, // bool
    'include_by_ip' => True, // bool
    'include_by_email_domain' => True, // bool
    'sso' => 'sso_example', // string
];


try {
    $result = $apiInstance->getPreBanSummary($tenant_id, $comment_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getPreBanSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---