---
ページの通知を有効または無効にします。ユーザーがページを購読している場合、新しいルートコメントに対して通知が作成され、また

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | クエリ | はい |  |
| urlId | string | クエリ | はい |  |
| url | string | クエリ | はい |  |
| pageTitle | string | クエリ | はい |  |
| subscribedOrUnsubscribed | string | パス | はい |  |
| sso | string | クエリ | いいえ |  |

## レスポンス

戻り値: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UpdateUserNotificationPageSubscriptionStatusResponse.java)

## 例

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスのインポート:
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
    String url = "url_example"; // String | 
    String pageTitle = "pageTitle_example"; // String | 
    String subscribedOrUnsubscribed = "subscribe"; // String | 
    String sso = "sso_example"; // String | 
    try {
      UpdateUserNotificationPageSubscriptionStatusResponse result = apiInstance.updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#updateUserNotificationPageSubscriptionStatus");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---