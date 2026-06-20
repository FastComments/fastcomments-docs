### 認証されたAPIの使用 (DefaultApi)

**重要:** You must set your API key on the ApiClient before making authenticated requests. If you don't, requests will fail with a 401 error.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // API クライアントを作成して設定します
        ApiClient apiClient = new ApiClient();

        // REQUIRED: Set your API key (get this from your FastComments dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 設定したクライアントで API インスタンスを作成します
        DefaultApi api = new DefaultApi(apiClient);

        // これで認証された API 呼び出しができます
        try {
            // 例: SSO ユーザーを追加
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // よくあるエラー:
            // - 401: API キーが欠落しているか無効です
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

### モデレーション API の使用 (ModerationApi)

The `ModerationApi` drives the moderator dashboard. Each method accepts an `sso` parameter identifying the SSO-authenticated moderator on whose behalf the request is made:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // 承認待ちのコメントを一覧表示
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### よくある問題

1. **401 "missing-api-key" error**: DefaultApi インスタンスを作成する前に `apiClient.setApiKey("YOUR_KEY")` を呼び出していることを確認してください。
2. **Wrong API class**: サーバー側の認証済みリクエストには `DefaultApi` を、クライアント側/パブリックなリクエストには `PublicApi` を使用してください。
3. **Null API key**: SDK は API キーが null の場合に認証を静かにスキップするため、401 エラーが発生します。