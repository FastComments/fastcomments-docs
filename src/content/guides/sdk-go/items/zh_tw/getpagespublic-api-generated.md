列出某租戶的頁面。FChat 桌面客戶端會使用此資料填充其房間列表。
需要在每個頁面的解析後自訂設定中，`enableFChat` 為 true。
需要 SSO 的頁面會根據請求用戶的群組存取權進行過濾。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| cursor | string | query | 否 | 不透明的分頁游標，從先前的請求回傳為 `nextCursor`。與相同的 `sortBy` 綁定。 |
| limit | integer | query | 否 | 1..200，預設 50 |
| q | string | query | 否 | 可選的不區分大小寫標題前綴篩選。 |
| sortBy | string | query | 否 | 排序方式。`updatedAt`（預設，依更新時間最新優先）、`commentCount`（留言數最多優先）、或 `title`（字母順序）。 |
| hasComments | boolean | query | 否 | 若為 true，僅回傳至少有一則留言的頁面。 |

## 回應

回傳：[`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_public_pages_response.go)

## 範例

[inline-code-attrs-start title = 'GetPagesPublic 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	cursor := "cursor_example" // string | 不透明的分頁游標，從先前的請求回傳為 `nextCursor`。與相同的 `sortBy` 綁定。 (可選)
	limit := int32(56) // int32 | 1..200，預設 50（可選）
	q := "q_example" // string | 可選的不區分大小寫標題前綴篩選。 (可選)
	sortBy := openapiclient.PagesSortBy("updatedAt") // PagesSortBy | 排序方式。`updatedAt`（預設，依更新時間最新優先）、`commentCount`（留言數最多優先）、或 `title`（字母順序）。 (可選)
	hasComments := true // bool | 若為 true，僅回傳至少有一則留言的頁面。 (可選)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetPagesPublic(context.Background(), tenantId).Cursor(cursor).Limit(limit).Q(q).SortBy(sortBy).HasComments(hasComments).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetPagesPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetPagesPublic`: GetPublicPagesResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetPagesPublic`: %v\n", resp)
}
[inline-code-end]