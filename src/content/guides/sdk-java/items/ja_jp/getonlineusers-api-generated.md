---
現在オンラインのページ閲覧者: 現在 websocket セッションがそのページを購読している人々。
anonCount + totalCount を返します（部屋全体の購読者数、列挙しない匿名閲覧者を含む）。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | ページのURL識別子（サーバー側でクリーンされます）。 |
| afterName | string | query | No | カーソル: 前回のレスポンスの nextAfterName を渡します。 |
| afterUserId | string | query | No | カーソルのタイブレーカー: 前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合、名前の同値によってエントリが欠落しないように必要です。 |

## レスポンス

戻り値: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## 例

[inline-code-attrs-start title = 'getOnlineUsers の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート:
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
    String urlId = "urlId_example"; // String | ページのURL識別子（サーバー側でクリーンされます）。
    String afterName = "afterName_example"; // String | カーソル: 前回のレスポンスの nextAfterName を渡します。
    String afterUserId = "afterUserId_example"; // String | カーソルのタイブレーカー: 前回のレスポンスの nextAfterUserId を渡します。afterName が設定されている場合、名前の同値によってエントリが欠落しないように必要です。
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---