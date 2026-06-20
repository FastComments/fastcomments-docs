---
테넌트의 페이지 목록을 반환합니다. FChat 데스크톱 클라이언트에서 룸 목록을 채우는 데 사용됩니다. 각 페이지에 대해 해결된 커스텀 구성에서 `enableFChat`이 true여야 합니다. SSO가 필요한 페이지는 요청 사용자의 그룹 접근 권한에 따라 필터링됩니다.

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| cursor | string | query | 아니요 | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 묶입니다. |
| limit | integer | query | 아니요 | 1..200, 기본값 50 |
| q | string | query | 아니요 | 선택적(대소문자 구분 없음) 제목 접두사 필터입니다. |
| sortBy | string | query | 아니요 | 정렬 순서. `updatedAt` (기본, 최신순), `commentCount` (댓글 많은 순), 또는 `title` (알파벳순). |
| hasComments | boolean | query | 아니요 | true인 경우 댓글이 하나 이상 있는 페이지만 반환합니다. |

## 응답

반환: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## 예제

[inline-code-attrs-start title = 'getPagesPublic 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 클래스 가져오기:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String cursor = "cursor_example"; // String | 이전 요청에서 `nextCursor`로 반환된 불투명한 페이지네이션 커서입니다. 동일한 `sortBy`에 묶입니다.
    Integer limit = 56; // Integer | 1..200, 기본값 50
    String q = "q_example"; // String | 선택적(대소문자 구분 없음) 제목 접두사 필터입니다.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | 정렬 순서. `updatedAt`(기본, 최신순), `commentCount`(댓글 많은 순), 또는 `title`(알파벳순).
    Boolean hasComments = true; // Boolean | true인 경우 댓글이 하나 이상 있는 페이지만 반환합니다.
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---