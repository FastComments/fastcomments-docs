### 使用已认证的 API (DefaultApi)

**Important:** 在发起需要认证的请求之前，您必须在 ApiClient 上设置您的 API key。如果不设置，请求将返回 401 错误。

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // 创建并配置 API 客户端
        ApiClient apiClient = new ApiClient();

        // 必需：设置您的 API key（从 FastComments 仪表板获取）
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // 使用已配置的客户端创建 API 实例
        DefaultApi api = new DefaultApi(apiClient);

        // 现在可以发起已认证的 API 调用
        try {
            // 示例：添加 SSO 用户
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
            // - 401：API key 缺失或无效
            // - 400：请求验证失败
        }
    }
}
```

### 使用公共 API (PublicApi)

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

### 使用审核 API (ModerationApi)

The `ModerationApi` drives the moderator dashboard. Each method accepts an `sso` parameter identifying the SSO-authenticated moderator on whose behalf the request is made:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // 列出等待审核的评论
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### 常见问题

1. **401 "missing-api-key" 错误**：确保在创建 DefaultApi 实例之前调用 `apiClient.setApiKey("YOUR_KEY")`。
2. **Wrong API class**：对于服务端需要认证的请求使用 `DefaultApi`，对于客户端/公共请求使用 `PublicApi`。
3. **Null API key**：SDK 将在 API key 为 null 时默默跳过认证，导致 401 错误。