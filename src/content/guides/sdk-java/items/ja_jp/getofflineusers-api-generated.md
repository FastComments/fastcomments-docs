そのページの過去のコメント投稿者（現在オンラインではないユーザー）。displayName でソートされます。
/users/online を使い果たした後に "Members" セクションを表示するためにこれを使用してください。
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい | ページのURL識別子（サーバー側でクリーンされます）。 |
| afterName | string | query | いいえ | カーソル: 前のレスポンスの nextAfterName を渡してください。 |
| afterUserId | string | query | いいえ | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡してください。afterName が設定されている場合、名前が同じエントリが落ちないように必須です。 |

## レスポンス

戻り値: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## 例

[inline-code-attrs-start title = 'getOfflineUsers の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | ページのURL識別子（サーバー側でクリーンされます）。
    String afterName = "afterName_example"; // String | カーソル: 前のレスポンスの nextAfterName を渡してください。
    String afterUserId = "afterUserId_example"; // String | カーソルのタイブレーカー: 前のレスポンスの nextAfterUserId を渡してください。afterName が設定されている場合、名前が同じエントリが落ちないように必須です。
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---