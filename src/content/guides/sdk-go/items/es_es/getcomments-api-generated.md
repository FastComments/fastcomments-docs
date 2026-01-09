## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sí |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |

## Respuesta

Devuelve: [`GetComments200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (opcional)
	limit := int32(56) // int32 |  (opcional)
	skip := int32(56) // int32 |  (opcional)
	asTree := true // bool |  (opcional)
	skipChildren := int32(56) // int32 |  (opcional)
	limitChildren := int32(56) // int32 |  (opcional)
	maxTreeDepth := int32(56) // int32 |  (opcional)
	urlId := "urlId_example" // string |  (opcional)
	userId := "userId_example" // string |  (opcional)
	anonUserId := "anonUserId_example" // string |  (opcional)
	contextUserId := "contextUserId_example" // string |  (opcional)
	hashTag := "hashTag_example" // string |  (opcional)
	parentId := "parentId_example" // string |  (opcional)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetComments`: GetComments200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]