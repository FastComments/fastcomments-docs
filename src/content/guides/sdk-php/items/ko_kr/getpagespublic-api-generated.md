테넌트의 페이지 목록을 조회합니다. FChat 데스크톱 클라이언트가 방 목록을 채우는 데 사용됩니다.
각 페이지의 확인된 커스텀 설정에서 `enableFChat`이 true여야 합니다.
SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한을 기준으로 필터링됩니다.

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| cursor | string | query | 아니요 | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 연결됩니다. |
| limit | integer | query | 아니요 | 1..200, 기본값 50 |
| q | string | query | 아니요 | 선택 사항인 대소문자를 구분하지 않는 제목 접두사 필터입니다. |
| sortBy | string | query | 아니요 | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순). |
| hasComments | boolean | query | 아니요 | true인 경우, 댓글이 하나 이상 있는 페이지만 반환합니다. |

## 응답

반환값: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## 예제

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 연결됩니다.
    'limit' => 56, // int | 1..200, 기본값 50
    'q' => 'q_example', // string | 선택 사항인 대소문자를 구분하지 않는 제목 접두사 필터입니다.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순).
    'has_comments' => True, // bool | true인 경우, 댓글이 하나 이상 있는 페이지만 반환합니다.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]