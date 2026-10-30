## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| sso | string | query | 아니오 |  |
| includeUserInfo | string | query | 아니오 | 설정하면 각 react id에 대한 사용자(아바타와 사용자 이름)를 반환합니다. "public"만 허용되며, 이는 비공개 프로필을 제외합니다. |

## 응답

반환: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetV2PageReacts.java)

## 예제

[inline-code-attrs-start title = 'getV2PageReacts 예제'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 
    String sso = "sso_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | 설정하면 각 react id에 대한 사용자(아바타와 사용자 이름)를 반환합니다. "public"만 허용되며, 이는 비공개 프로필을 제외합니다.
    try {
      GetV2PageReacts result = apiInstance.getV2PageReacts(tenantId, urlId)
            .sso(sso)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getV2PageReacts");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]