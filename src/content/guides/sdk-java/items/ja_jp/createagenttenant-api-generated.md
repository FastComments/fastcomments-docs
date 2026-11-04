人間のサインアップなしで、AIエージェント用の新しいトライアルアカウントを作成します。この呼び出しにAPIキーは必要ありません。

レスポンスには、テナントID、REST APIおよびMCPサーバーに対して即座に使用できるAPIキー、そしてクレームURLが含まれます。クレームURLは、あなたが作業している人間に渡してください。FastCommentsにログインした状態でこのURLを開くと、そのアカウントがその人に紐付けられます。クレームされていないアカウントとそのキーは、作成から72時間後に削除されます。クレームされるまで、アカウントには標準のトライアル制限が適用されます。

## レスポンス

戻り値: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## 例

[inline-code-attrs-start title = 'createAgentTenant の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]