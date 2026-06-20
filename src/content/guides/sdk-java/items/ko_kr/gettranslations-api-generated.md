## 매개변수

| 이름 | 형식 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| namespace | string | path | 예 |  |
| component | string | path | 예 |  |
| locale | string | query | 아니오 |  |
| useFullTranslationIds | boolean | query | 아니오 |  |

## 응답

반환: [`GetTranslationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTranslationsResponse.java)

## 예제

[inline-code-attrs-start title = 'getTranslations 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String namespace = "namespace_example"; // String | 
    String component = "component_example"; // String | 
    String locale = "locale_example"; // String | 
    Boolean useFullTranslationIds = true; // Boolean | 
    try {
      GetTranslationsResponse result = apiInstance.getTranslations(namespace, component)
            .locale(locale)
            .useFullTranslationIds(useFullTranslationIds)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getTranslations");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---