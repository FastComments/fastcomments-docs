## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| page | number | No |  |
| count | number | No |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## 응답

Returns: [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## 예시

[inline-code-attrs-start title = 'getApiComments 예시'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // 페이지
    25,                    // 카운트
    "feedback",           // 텍스트 검색
    "192.168.1.100",      // 댓글 IP
    "approved",           // 필터
    "hasReplies",         // 검색 필터
    "dateDesc",           // 정렬
    false,                // 데모
    "tenant-abc123",      // 테넌트 ID
    "sso-token-xyz"       // SSO
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]

---