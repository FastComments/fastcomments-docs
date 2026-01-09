## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| id | string | path | 예 |  |
| userId | string | query | 아니요 |  |

## 응답

반환: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteSubscriptionAPIResponse.java)

## 예제

[inline-code-attrs-start title = 'deleteSubscription 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 클래스 가져오기:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // API 키 인증 구성: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 다음 줄의 주석을 제거하여 API 키 접두사를 설정합니다(예: "Token"). 기본값은 null입니다.
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // 문자열 | 
    String id = "id_example"; // 문자열 | 
    String userId = "userId_example"; // 문자열 | 
    try {
      DeleteSubscriptionAPIResponse result = apiInstance.deleteSubscription(tenantId, id)
            .userId(userId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#deleteSubscription");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]