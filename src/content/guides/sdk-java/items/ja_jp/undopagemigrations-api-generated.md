Queues a batch job that reverses every migration from one migrate-pages job, newest first. Available for 90 days.  
1つの migrate‑pages ジョブからのすべてのマイグレーションを逆順（最新のものから）に取り消すバッチジョブをキューに入れます。利用可能期間は 90 日です。  
Records changed since the migration are left alone, so a later edit is never overwritten.  
マイグレーション以降に変更されたレコードはそのまま残るため、後からの編集が上書きされることはありません。  
Poll GET /api/v1/batch-jobs/{batchJobId} for the restored/skipped counts.  
復元/スキップされた件数を取得するには、GET /api/v1/batch-jobs/{batchJobId} をポーリングしてください。

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| undoId | string | path | はい |  |

## Response

返却: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## Example

[inline-code-attrs-start title = 'undoPageMigrations の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // API キー認証を設定: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 以下の行のコメントを外すと、API キーにプレフィックス（例: "Token"）を設定できます（デフォルトは null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String undoId = "undoId_example"; // String | 
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.undoPageMigrations(tenantId, undoId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#undoPageMigrations");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---