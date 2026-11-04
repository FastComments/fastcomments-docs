테넌트의 대량 사용자 정보입니다. userIds가 주어지면 User / SSOUser에서 표시 정보를 반환합니다.
presence 이벤트를 통해 방금 나타난 사용자를 풍부하게 하기 위해 댓글 위젯에서 사용됩니다.
페이지 컨텍스트가 없으므로 프라이버시가 일관되게 적용됩니다(비공개 프로필은 마스킹됩니다).

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| ids | string | query | 예 | 쉼표로 구분된 userIds입니다. |

## 응답

반환값: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## 예제

[inline-code-attrs-start title = 'getUsersInfo 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 http 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 이는 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | 쉼표로 구분된 userIds입니다.


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]