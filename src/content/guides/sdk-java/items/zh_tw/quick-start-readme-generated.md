### 使用已驗證的 API（DefaultApi）

**重要：** 在發出已驗證的請求之前，您必須在 ApiClient 上設定 API 金鑰。若未設定，請求將會以 401 錯誤失敗。

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // 建立並設定 API 用戶端
        ApiClient apiClient = new ApiClient();

        // 必須：設定您的 API 金鑰（從您的 FastComments 儀表板取得）
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 使用已設定的用戶端建立 API 實例
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
            // - 401：API 金鑰遺失或無效
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

### 常見問題

1. **401 "missing-api-key" error**：請確保在建立 DefaultApi 實例之前呼叫 `apiClient.setApiKey("YOUR_KEY")`。
2. **Wrong API class**：對於伺服器端的已驗證請求使用 `DefaultApi`，對於用戶端/公開請求使用 `PublicApi`。
3. **Null API key**：若 API 金鑰為 null，SDK 會靜默地跳過驗證，導致 401 錯誤。