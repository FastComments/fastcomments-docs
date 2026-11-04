### 認証付きAPIの利用 (DefaultApi)

**重要:** 認証が必要なリクエストを行う前に、ApiClientにAPIキーを設定する必要があります。設定しない場合、リクエストは401エラーで失敗します。

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // APIクライアントを作成して設定
        ApiClient apiClient = new ApiClient();

        // 必須: APIキーを設定(FastCommentsダッシュボードから取得)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 設定済みのクライアントでAPIインスタンスを作成
        DefaultApi api = new DefaultApi(apiClient);

        // これで認証付きのAPI呼び出しが可能になります
        try {
            // 例: SSOユーザーを追加
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
            // - 401: APIキーが未設定または無効
            // - 400: リクエストの検証に失敗
        }
    }
}
```

### 公開APIの利用 (PublicApi)

公開エンドポイントは認証を必要としません:

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

### モデレーションAPIの利用 (ModerationApi)

`ModerationApi` はモデレーターダッシュボードを動かします。各メソッドは、リクエストの対象となるSSO認証済みモデレーターを識別する `sso` パラメーターを受け取ります:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // モデレーション待ちのコメントを一覧表示
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### よくある問題

1. **401 "missing-api-key" エラー**: DefaultApiインスタンスを作成する前に、必ず `apiClient.setApiKey("YOUR_KEY")` を呼び出してください。
2. **APIクラスの誤り**: サーバーサイドの認証付きリクエストには `DefaultApi` を、クライアントサイド/公開リクエストには `PublicApi` を使用してください。
3. **APIキーがnull**: APIキーがnullの場合、SDKは認証を黙ってスキップするため、401エラーが発生します。