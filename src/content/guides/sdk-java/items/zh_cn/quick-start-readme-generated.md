### 使用已认证的 API（DefaultApi）

**重要：** 在进行需要认证的请求之前，您必须在 ApiClient 上设置您的 API 密钥。如果不这样，请求将返回 401 错误。

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // 创建并配置 API 客户端
        ApiClient apiClient = new ApiClient();

        // 必需：设置您的 API 密钥（可从 FastComments 仪表板获取）
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 使用已配置的客户端创建 API 实例
        DefaultApi api = new DefaultApi(apiClient);

        // 现在您可以进行已认证的 API 调用
        try {
            // 示例：添加一个 SSO 用户
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // 常见错误：
            // - 401：API 密钥缺失或无效
            // - 400：请求验证失败
        }
    }
}
```

### 使用公共 API（PublicApi）

公共端点不需要认证：

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

### 常见问题

1. **401 "missing-api-key" 错误**：确保在创建 DefaultApi 实例之前调用 `apiClient.setApiKey("YOUR_KEY")`。
2. **错误的 API 类**：对服务器端需要认证的请求使用 `DefaultApi`，对客户端/公共请求使用 `PublicApi`。
3. **空 API 密钥**：如果 API 密钥为 null，SDK 会静默跳过认证，导致 401 错误。