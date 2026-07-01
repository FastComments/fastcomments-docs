## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| sso | string | query | No |  |

## 回應

返回：[`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## 範例

[inline-code-attrs-start title = 'postBanUserUndo 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    String tenantId = "tenantId_example"; // String | 
    BanUserUndoParams banUserUndoParams = new BanUserUndoParams(); // BanUserUndoParams | 
    String sso = "sso_example"; // String | 
    try {
      APIEmptyResponse result = apiInstance.postBanUserUndo(tenantId, banUserUndoParams)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postBanUserUndo");
      // 呼叫 ModerationApi#postBanUserUndo 時發生例外
      System.err.println("Status code: " + e.getCode());
      // 狀態碼: 
      System.err.println("Reason: " + e.getResponseBody());
      // 原因: 
      System.err.println("Response headers: " + e.getResponseHeaders());
      // 回應標頭: 
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---