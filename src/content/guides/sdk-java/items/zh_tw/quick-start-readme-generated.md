### 使用已驗證的 API（DefaultApi）

**重要：** 您必須在 ApiClient 上設定您的 API 金鑰，才能進行已驗證的請求。如果未設定，請求將會以 401 錯誤失敗。

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // 建立並設定 API client
        ApiClient apiClient = new ApiClient();

        // 必要：設定您的 API 金鑰（可從您的 FastComments 控制台取得）
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 使用已設定的 client 建立 API 實例
        DefaultApi api = new DefaultApi(apiClient);

        // 現在您可以進行已驗證的 API 呼叫
        try {
            // 範例：新增 SSO 使用者
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // 常見錯誤：
            // - 401：API key 遺失或無效
            // - 400：請求驗證失敗
        }
    }
}
```

### 使用公開 API（PublicApi）

公開端點不需要驗證：

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

### 使用審核 API（ModerationApi）

`ModerationApi` 驅動版主儀表板。每個方法接受一個 `sso` 參數，用以識別代表哪位透過 SSO 驗證的版主發出請求：

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // 列出等待審核的留言
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### 常見問題

1. **401 "missing-api-key" error**：請確認在建立 DefaultApi 實例之前有呼叫 `apiClient.setApiKey("YOUR_KEY")`。
2. **Wrong API class**：伺服器端已驗證的請求請使用 `DefaultApi`，用戶端/公開請求請使用 `PublicApi`。
3. **Null API key**：若 API key 為 null，SDK 會悄悄跳過驗證，導致 401 錯誤。