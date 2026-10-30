コメント、ページ、および関連レコードをある urlId またはドメインから別の urlId またはドメインへ移動するバッチジョブをキューに入れます。  
"urlId" マイグレーションは正確な urlId のみ一致します。"domain" マイグレーションはすべての urlId と url 内の値を検索/置換します。  
GET /api/v1/batch-jobs/{batchJobId} をポーリングします：完了すると、マイグレーションごとの結果と undoId が返されます。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## Example

[inline-code-attrs-start title = 'migratePages の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // APIキー認証を設定: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 以下の行のコメントを解除して、APIキーのプレフィックスを設定します（例: "Token"、デフォルトは null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    MigrateAPIPagesData migrateAPIPagesData = new MigrateAPIPagesData(); // MigrateAPIPagesData | 
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.migratePages(tenantId, migrateAPIPagesData)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#migratePages");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]