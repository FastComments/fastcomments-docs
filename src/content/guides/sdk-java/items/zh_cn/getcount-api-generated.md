---
## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| text-search | string | query | 否 |  |
| byIPFromComment | string | query | 否 |  |
| filter | string | query | 否 |  |
| searchFilters | string | query | 否 |  |
| demo | boolean | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationAPICountCommentsResponse.java)

## 示例

[inline-code-attrs-start title = 'getCount 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 导入类：
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
    String textSearch = "textSearch_example"; // 字符串 | 
    String byIPFromComment = "byIPFromComment_example"; // 字符串 | 
    String filter = "filter_example"; // 字符串 | 
    String searchFilters = "searchFilters_example"; // 字符串 | 
    Boolean demo = true; // 布尔 | 
    String sso = "sso_example"; // 字符串 | 
    try {
      ModerationAPICountCommentsResponse result = apiInstance.getCount()
            .textSearch(textSearch)
            .byIPFromComment(byIPFromComment)
            .filter(filter)
            .searchFilters(searchFilters)
            .demo(demo)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#getCount");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---