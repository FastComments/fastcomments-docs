---
テナントのページ一覧を返します。FChat デスクトップクライアントがルーム一覧を生成するために使用します。
各ページについて解決されたカスタム設定で `enableFChat` が true である必要があります。
SSO を必要とするページは、要求元ユーザーのグループアクセスに基づいてフィルタされます。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 以前のリクエストから `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に結びついています。 |
| limit | integer | query | No | 1..200、デフォルト 50 |
| q | string | query | No | 省略可の大文字小文字を区別しないタイトル先頭一致フィルタ。 |
| sortBy | string | query | No | 並び順。 `updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多い順）、または `title`（アルファベット順）。 |
| hasComments | boolean | query | No | true の場合、少なくとも1件のコメントがあるページのみを返します。 |

## Response

Returns: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## Example

[inline-code-attrs-start title = 'GetPagesPublic の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	cursor := "cursor_example" // string | 以前のリクエストから `nextCursor` として返された不透明なページネーションカーソル。同じ `sortBy` に結びついています。 (省略可)
	limit := int32(56) // int32 | 1..200、デフォルト 50 (省略可)
	q := "q_example" // string | 省略可の大文字小文字を区別しないタイトル先頭一致フィルタ。 (省略可)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | 並び順。`updatedAt`（デフォルト、最新が先）、`commentCount`（コメント数が多い順）、または`title`（アルファベット順）。 (省略可)
	hasComments := true // bool | true の場合、少なくとも1件のコメントがあるページのみを返します。 (省略可)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetPagesPublic` のレスポンス: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]

---