## 매개변수

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| commentIds | string | query | 예 | 쉼표로 구분된 댓글 ID 목록. |
| sso | string | query | 아니요 |  |

## 응답

반환값: [`CheckedCommentsForBlocked200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CheckedCommentsForBlocked200Response.java)

## 예제

[inline-code-attrs-start title = 'checkedCommentsForBlocked 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String commentIds = "commentIds_example"; // String | 쉼표로 구분된 댓글 ID 목록.
    String sso = "sso_example"; // String | 
    try {
      CheckedCommentsForBlocked200Response result = apiInstance.checkedCommentsForBlocked(tenantId, commentIds)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#checkedCommentsForBlocked");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---