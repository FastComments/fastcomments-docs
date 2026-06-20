テナントのページを一覧表示します。FChat デスクトップクライアントがルームリストを生成するために使用します。各ページの解決されたカスタム設定で `enableFChat` が true である必要があります。SSO を必要とするページは、リクエストユーザーのグループアクセスに基づいてフィルターされます。

## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| cursor | string | query | いいえ | 前のリクエストから返された `nextCursor` の不透明なページネーションカーソル。`sortBy` と紐づきます。 |
| limit | integer | query | いいえ | 1..200、デフォルト 50 |
| q | string | query | いいえ | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルター。 |
| sortBy | string | query | いいえ | ソート順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | いいえ | true の場合、少なくとも1件のコメントがあるページのみを返します。 |

## レスポンス

戻り値: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## 例

[inline-code-attrs-start title = 'getPagesPublic の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String cursor = "cursor_example"; // String | 前のリクエストから返された `nextCursor` の不透明なページネーションカーソル。 同じ `sortBy` に紐づきます。
    Integer limit = 56; // Integer | 1..200、デフォルト 50
    String q = "q_example"; // String | オプションの大文字小文字を区別しないタイトルのプレフィックスフィルタ。
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | ソート順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。
    Boolean hasComments = true; // Boolean | true の場合、少なくとも1件のコメントがあるページのみを返します。
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]