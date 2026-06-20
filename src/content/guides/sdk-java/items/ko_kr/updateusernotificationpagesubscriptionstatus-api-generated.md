---
페이지에 대한 알림을 활성화하거나 비활성화합니다. 사용자가 페이지를 구독하면 알림이 생성되며
새 루트 댓글에 대한 알림 등이 생성됩니다.

## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| url | string | query | Yes |  |
| pageTitle | string | query | Yes |  |
| subscribedOrUnsubscribed | string | path | Yes |  |
| sso | string | query | No |  |

## 응답

반환: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UpdateUserNotificationPageSubscriptionStatusResponse.java)

## 예제

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // 문자열 | 
    String url = "url_example"; // 문자열 | 
    String pageTitle = "pageTitle_example"; // 문자열 | 
    String subscribedOrUnsubscribed = "subscribe"; // 문자열 | 
    String sso = "sso_example"; // 문자열 | 
    try {
      UpdateUserNotificationPageSubscriptionStatusResponse result = apiInstance.updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#updateUserNotificationPageSubscriptionStatus");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---