テナントのユーザー情報を一括取得します。userIds を指定すると、User / SSOUser から表示用の情報を返します。
コメントウィジェットが presence event で新たに出現したユーザーを補完するために使用されます。
ページコンテキストがないため、プライバシーは一律に適用されます（非公開プロフィールはマスクされます）。

## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| ids | string | query | はい | コンマ区切りの userIds. |

## レスポンス

戻り値: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersInfoResponse.java)

## 例

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String ids = "ids_example"; // String | コンマ区切りの userIds.
    try {
      PageUsersInfoResponse result = apiInstance.getUsersInfo(tenantId, ids)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUsersInfo");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---