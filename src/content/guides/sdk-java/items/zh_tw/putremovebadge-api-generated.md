## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| badgeId | string | query | Yes |  |
| userId | string | query | No |  |
| commentId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## 回應

返回：[`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/RemoveUserBadgeResponse.java)

## 範例

[inline-code-attrs-start title = 'putRemoveBadge 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 匯入類別:
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
    String tenantId = "tenantId_example"; // 字串 | 
    String badgeId = "badgeId_example"; // 字串 | 
    String userId = "userId_example"; // 字串 | 
    String commentId = "commentId_example"; // 字串 | 
    String broadcastId = "broadcastId_example"; // 字串 | 
    String sso = "sso_example"; // 字串 | 
    try {
      RemoveUserBadgeResponse result = apiInstance.putRemoveBadge(tenantId, badgeId)
            .userId(userId)
            .commentId(commentId)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("呼叫 ModerationApi#putRemoveBadge 時的例外");
      System.err.println("狀態碼: " + e.getCode());
      System.err.println("原因: " + e.getResponseBody());
      System.err.println("回應標頭: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]