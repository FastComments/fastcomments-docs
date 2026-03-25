## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| userId | string | query | 예 |  |
| id | string | path | 예 |  |

## 응답

반환: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ChangeTicketState200Response.java)

## 예제

[inline-code-attrs-start title = 'changeTicketState 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // API 키에 접두어를 설정하려면 다음 줄의 주석을 해제하세요. 예: "Token" (기본값은 null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    String id = "id_example"; // String | 
    ChangeTicketStateBody changeTicketStateBody = new ChangeTicketStateBody(); // ChangeTicketStateBody | 
    try {
      ChangeTicketState200Response result = apiInstance.changeTicketState(tenantId, userId, id, changeTicketStateBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#changeTicketState");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---