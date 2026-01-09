## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| postIds | array | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetUserReactsPublic200Response.java)

## Пример

[inline-code-attrs-start title = 'getUserReactsPublic Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увези класе:
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
    List<String> postIds = Arrays.asList(); // List<String> | 
    String sso = "sso_example"; // String | 
    try {
      GetUserReactsPublic200Response result = apiInstance.getUserReactsPublic(tenantId)
            .postIds(postIds)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUserReactsPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]