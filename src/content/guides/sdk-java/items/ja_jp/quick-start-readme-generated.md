### 認証済み API の使用 (DefaultApi)

**重要:** 認証付きリクエストを行う前に ApiClient に API キーを設定する必要があります。設定しないとリクエストは 401 エラーで失敗します。

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // API クライアントを作成して設定する
        ApiClient apiClient = new ApiClient();

        // 必須: API キーを設定してください（FastComments ダッシュボードで取得）
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 設定済みのクライアントで API インスタンスを作成する
        DefaultApi api = new DefaultApi(apiClient);

        // これで認証された API 呼び出しが可能になります
        try {
            // 例: SSO ユーザーを追加する
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // 一般的なエラー:
            // - 401: API キーがないか無効です
            // - 400: リクエストの検証に失敗しました
        }
    }
}
```

### パブリック API の使用 (PublicApi)

パブリックエンドポイントは認証を必要としません:

```java
import com.fastcomments.api.PublicApi;
import com.fastcomments.invoker.ApiException;

PublicApi publicApi = new PublicApi();

try {
    var response = publicApi.getCommentsPublic("YOUR_TENANT_ID", "page-url-id")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### よくある問題

1. **401 "missing-api-key" エラー**: DefaultApi インスタンスを作成する前に `apiClient.setApiKey("YOUR_KEY")` を呼び出していることを確認してください。
2. **誤った API クラス**: サーバー側の認証付きリクエストには `DefaultApi` を、クライアント側/パブリックなリクエストには `PublicApi` を使用してください。
3. **API キーが null**: API キーが null の場合、SDK は認証を黙ってスキップするため、401 エラーになります。