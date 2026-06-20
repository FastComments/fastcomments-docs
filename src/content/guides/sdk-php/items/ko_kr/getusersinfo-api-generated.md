---
테넌트의 대량 사용자 정보. userIds가 주어지면 User / SSOUser의 표시 정보를 반환합니다.
댓글 위젯에서 프레즌스 이벤트로 방금 나타난 사용자들을 보강하는 데 사용됩니다.
페이지 컨텍스트 없음: 개인정보 보호는 일관되게 적용됩니다(비공개 프로필은 마스킹됩니다).

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| ids | string | query | 예 | 쉼표로 구분된 userIds. |

## 응답

반환: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## 예제

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 커스텀 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이것은 선택사항이며, 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | 쉼표로 구분된 userIds.

try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---