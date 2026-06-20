---
## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| badgeId | string | query | 是 |  |
| userId | string | query | 否 |  |
| commentId | string | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AwardUserBadgeResponse.java)

## 範例

[inline-code-attrs-start title = 'putAwardBadge 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 匯入類別：
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
    String badgeId = "badgeId_example"; // 字串 | 
    String userId = "userId_example"; // 字串 | 
    String commentId = "commentId_example"; // 字串 | 
    String broadcastId = "broadcastId_example"; // 字串 | 
    String sso = "sso_example"; // 字串 | 
    try {
      AwardUserBadgeResponse result = apiInstance.putAwardBadge(badgeId)
            .userId(userId)
            .commentId(commentId)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#putAwardBadge");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---