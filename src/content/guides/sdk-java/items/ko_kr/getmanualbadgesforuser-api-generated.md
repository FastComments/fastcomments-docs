## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgesUserId | string | query | 아니오 |  |
| commentId | string | query | 아니오 |  |
| sso | string | query | 아니오 |  |

## 응답

반환: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetUserManualBadgesResponse.java)

## 예제

[inline-code-attrs-start title = 'getManualBadgesForUser 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 클래스 임포트:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String badgesUserId = "badgesUserId_example"; // 문자열 | 
    String commentId = "commentId_example"; // 문자열 | 
    String sso = "sso_example"; // 문자열 | 
    try {
      GetUserManualBadgesResponse result = apiInstance.getManualBadgesForUser()
            .badgesUserId(badgesUserId)
            .commentId(commentId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#getManualBadgesForUser");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]