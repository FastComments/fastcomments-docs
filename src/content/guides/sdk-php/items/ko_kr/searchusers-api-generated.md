## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| usernameStartsWith | string | query | 아니오 |  |
| mentionGroupIds | array | query | 아니오 |  |
| sso | string | query | 아니오 |  |
| searchSection | string | query | 아니오 |  |

## 응답

반환: [`SearchUsersResult`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/SearchUsersResult.php)

## 예제

[inline-code-attrs-start title = 'searchUsers 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항이며 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string
$username_starts_with = 'username_starts_with_example'; // string
$mention_group_ids = array('mention_group_ids_example'); // string[]
$sso = 'sso_example'; // string
$search_section = 'search_section_example'; // string

try {
    $result = $apiInstance->searchUsers($tenant_id, $url_id, $username_starts_with, $mention_group_ids, $sso, $search_section);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->searchUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]