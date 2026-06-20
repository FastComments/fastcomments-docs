---
현재 온라인 상태가 아닌 페이지의 과거 댓글 작성자들입니다. displayName으로 정렬됩니다.
/users/online을 모두 사용한 후 "Members" 섹션을 렌더링할 때 이것을 사용하세요.
commenterName에 대한 커서 페이지네이션: 서버는 부분 {tenantId, urlId, commenterName} 인덱스를 순회합니다.
afterName부터 $gt를 사용하여 앞으로 인덱스를 조회하며, $skip 비용이 없습니다.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 | 페이지 URL 식별자 (서버 측에서 정리됨). |
| afterName | string | query | 아니오 | 커서: 이전 응답의 nextAfterName을 전달합니다. |
| afterUserId | string | query | 아니오 | 커서 동점 해소: 이전 응답의 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름 중복으로 항목이 누락되지 않도록 필요합니다. |

## Response

반환: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Example

[inline-code-attrs-start title = 'getOfflineUsers 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    String urlId = "urlId_example"; // String | 페이지 URL 식별자 (서버 측에서 정리됨).
    String afterName = "afterName_example"; // String | 커서: 이전 응답의 nextAfterName을 전달합니다.
    String afterUserId = "afterUserId_example"; // String | 커서 동점 해소: 이전 응답의 nextAfterUserId를 전달합니다. afterName이 설정된 경우 이름 중복으로 항목이 누락되지 않도록 필요합니다.
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---