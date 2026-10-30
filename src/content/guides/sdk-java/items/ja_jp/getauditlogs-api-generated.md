## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい | 監査ログを取得するテナント。 |
| limit | integer | query | いいえ | 返却するログの数。最大 10000、デフォルトは 1000。 |
| skip | integer | query | いいえ | ページングのためにスキップするログの数。 |
| order | string | query | いいえ | イベント時間のソート方向。デフォルトは DESC（新しいものが先）。 |
| after | integer | query | いいえ |  |
| before | integer | query | いいえ |  |
| username | string | query | いいえ | このユーザー名で実行されたイベントのみを返す。 |
| ip | string | query | いいえ | この IP アドレスからのイベントのみを返す。 |
| crudType | string | query | いいえ | このタイプ（c、r、u、d、または login）のイベントのみを返す。 |
| resourceName | string | query | いいえ | このリソース（例: User または Moderator）のイベントのみを返す。 |
| targetId | string | query | いいえ | 対象オブジェクトがこの ID を持つイベントのみを返す。 |
| target | string | query | いいえ | 対象オブジェクトのラベルに対する大文字小文字を区別しない部分文字列一致（例: ユーザー名またはメール）。 |
| includeManagedTenants | boolean | query | いいえ | このテナントが管理するテナントからのイベントも返す。 |

## レスポンス

返却: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## 例

[inline-code-attrs-start title = 'getAuditLogs の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // 以下の行のコメントを外すと、API キーのプレフィックスを設定できます（例: "Token"、デフォルトは null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 監査ログを取得するテナント。
    Integer limit = 56; // Integer | 返却するログの数。最大 10000、デフォルトは 1000。
    Integer skip = 56; // Integer | ページングのためにスキップするログの数。
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | イベント時間のソート方向。デフォルトは DESC（新しいものが先）。
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | このユーザー名で実行されたイベントのみを返す。
    String ip = "ip_example"; // String | この IP アドレスからのイベントのみを返す。
    String crudType = "crudType_example"; // String | このタイプ（c、r、u、d、または login）のイベントのみを返す。
    String resourceName = "resourceName_example"; // String | このリソース（例: User または Moderator）のイベントのみを返す。
    String targetId = "targetId_example"; // String | 対象オブジェクトがこの ID を持つイベントのみを返す。
    String target = "target_example"; // String | 対象オブジェクトのラベルに対する大文字小文字を区別しない部分文字列一致（例: ユーザー名またはメール）。
    Boolean includeManagedTenants = true; // Boolean | このテナントが管理するテナントからのイベントも返す。
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]