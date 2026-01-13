## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| page | integer | query | Ні |  |
| limit | integer | query | Ні |  |
| skip | integer | query | Ні |  |
| asTree | boolean | query | Ні |  |
| skipChildren | integer | query | Ні |  |
| limitChildren | integer | query | Ні |  |
| maxTreeDepth | integer | query | Ні |  |
| urlId | string | query | Ні |  |
| userId | string | query | Ні |  |
| anonUserId | string | query | Ні |  |
| contextUserId | string | query | Ні |  |
| hashTag | string | query | Ні |  |
| parentId | string | query | Ні |  |
| direction | string | query | Ні |  |

## Відповідь

Повертає: [`GetComments200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_200_response.go)

## Приклад

[inline-code-attrs-start title = 'Приклад GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/client"
)

func main() {
	tenantId := "tenantId_example" // string | 
	page := int32(56) // int32 |  (необов'язково)
	limit := int32(56) // int32 |  (необов'язково)
	skip := int32(56) // int32 |  (необов'язково)
	asTree := true // bool |  (необов'язково)
	skipChildren := int32(56) // int32 |  (необов'язково)
	limitChildren := int32(56) // int32 |  (необов'язково)
	maxTreeDepth := int32(56) // int32 |  (необов'язково)
	urlId := "urlId_example" // string |  (необов'язково)
	userId := "userId_example" // string |  (необов'язково)
	anonUserId := "anonUserId_example" // string |  (необов'язково)
	contextUserId := "contextUserId_example" // string |  (необов'язково)
	hashTag := "hashTag_example" // string |  (необов'язково)
	parentId := "parentId_example" // string |  (необов'язково)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (необов'язково)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// відповідь з `GetComments`: GetComments200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]