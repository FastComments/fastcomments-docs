List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 이전 요청에서 `nextCursor` 로 반환된 불투명 페이지네이션 커서. 동일한 `sortBy`와 연결됩니다. |
| limit | integer | query | No | 1..200, 기본값 50 |
| q | string | query | No | 선택적 대소문자 구분 없는 제목 접두사 필터. |
| sortBy | string | query | No | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글이 많은 순), 또는 `title` (알파벳 순). |
| hasComments | boolean | query | No | true이면 최소 하나 이상의 댓글이 있는 페이지만 반환합니다. |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## 예시

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // 사용자 정의 HTTP 클라이언트를 사용하려면 `GuzzleHttp\ClientInterface`를 구현하는 클라이언트를 전달하십시오.
    // 이는 선택 사항이며, 기본값으로 `GuzzleHttp\Client`가 사용됩니다.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // 문자열
$options = [
    'cursor' => 'cursor_example', // 문자열 | 이전 요청에서 `nextCursor` 로 반환된 불투명 페이지네이션 커서. 동일한 `sortBy`와 연결됩니다.
    'limit' => 56, // 정수 | 1..200, 기본값 50
    'q' => 'q_example', // 문자열 | 선택적 대소문자 구분 없는 제목 접두사 필터.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | 정렬 순서. `updatedAt` (기본값, 최신 순), `commentCount` (댓글이 많은 순), 또는 `title` (알파벳 순).
    'has_comments' => True, // 불리언 | true이면 최소 하나 이상의 댓글이 있는 페이지만 반환합니다.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]