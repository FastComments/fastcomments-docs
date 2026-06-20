테넌트의 페이지를 나열합니다. FChat 데스크톱 클라이언트가 룸 목록을 채우기 위해 사용합니다.
각 페이지에 대한 해결된 커스텀 구성에서 `enableFChat`이 true여야 합니다.
SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 이전 요청으로부터 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 묶입니다. |
| limit | integer | query | No | 1..200, 기본값 50 |
| q | string | query | No | 선택적 대소문자 구분 없는 제목 접두사 필터. |
| sortBy | string | query | No | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 수 많은 순), 또는 `title` (사전순). |
| hasComments | boolean | query | No | true인 경우 최소 하나의 댓글이 있는 페이지만 반환합니다. |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## 예제

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하세요.
    // 선택 사항입니다. 기본적으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 묶입니다.
$limit = 56; // int | 1..200, 기본값 50
$q = 'q_example'; // string | 선택적 대소문자 구분 없는 제목 접두사 필터.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신순), `commentCount` (댓글 수 많은 순), 또는 `title` (사전순).
$has_comments = True; // bool | true인 경우 최소 하나의 댓글이 있는 페이지만 반환합니다.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]