---
req
tenantId
afterId

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 예 |  |
| afterId | string | query | 아니오 |  |
| limit | integer | query | 아니오 |  |
| tags | array | query | 아니오 |  |

## 응답

반환: [`GetFeedPosts200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetFeedPosts200Response.java)

## 예제

[inline-code-attrs-start title = 'getFeedPosts 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // API 키에 접두어를 설정하려면 다음 줄의 주석을 제거하세요. 예: "Token" (기본값: null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String afterId = "afterId_example"; // String | 
    Integer limit = 56; // Integer | 
    List<String> tags = Arrays.asList(); // List<String> | 
    try {
      GetFeedPosts200Response result = apiInstance.getFeedPosts(tenantId)
            .afterId(afterId)
            .limit(limit)
            .tags(tags)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getFeedPosts");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---