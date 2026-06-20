현재 온라인 상태인 페이지 뷰어: 웹소켓 세션이 현재 해당 페이지를 구독하고 있는 사람들입니다.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 | 페이지 URL 식별자(서버 측에서 정리됨). |
| afterName | string | query | 아니요 | 커서: 이전 응답의 nextAfterName을 전달하세요. |
| afterUserId | string | query | 아니요 | 커서 동점 해소자: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 동일한 항목이 누락되지 않도록 필요합니다. |

## 응답

반환: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## 예제

[inline-code-attrs-start title = 'getOnlineUsers 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String tenantId = "tenantId_example"; // 문자열 | 
    String urlId = "urlId_example"; // 문자열 | 페이지 URL 식별자(서버 측에서 정리됨).
    String afterName = "afterName_example"; // 문자열 | 커서: 이전 응답의 nextAfterName을 전달하세요.
    String afterUserId = "afterUserId_example"; // 문자열 | 커서 동점 해소자: 이전 응답의 nextAfterUserId를 전달하세요. afterName이 설정된 경우 이름이 동일한 항목이 누락되지 않도록 필요합니다.
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---