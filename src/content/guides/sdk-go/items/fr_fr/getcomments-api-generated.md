## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| page | integer | query | Non |  |
| limit | integer | query | Non |  |
| skip | integer | query | Non |  |
| asTree | boolean | query | Non |  |
| skipChildren | integer | query | Non |  |
| limitChildren | integer | query | Non |  |
| maxTreeDepth | integer | query | Non |  |
| urlId | string | query | Non |  |
| userId | string | query | Non |  |
| anonUserId | string | query | Non |  |
| contextUserId | string | query | Non |  |
| hashTag | string | query | Non |  |
| parentId | string | query | Non |  |
| direction | string | query | Non |  |

## Réponse

Renvoie : [`GetComments200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetComments'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (optionnel)
	limit := int32(56) // int32 |  (optionnel)
	skip := int32(56) // int32 |  (optionnel)
	asTree := true // bool |  (optionnel)
	skipChildren := int32(56) // int32 |  (optionnel)
	limitChildren := int32(56) // int32 |  (optionnel)
	maxTreeDepth := int32(56) // int32 |  (optionnel)
	urlId := "urlId_example" // string |  (optionnel)
	userId := "userId_example" // string |  (optionnel)
	anonUserId := "anonUserId_example" // string |  (optionnel)
	contextUserId := "contextUserId_example" // string |  (optionnel)
	hashTag := "hashTag_example" // string |  (optionnel)
	parentId := "parentId_example" // string |  (optionnel)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (optionnel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetComments(context.Background()).TenantId(tenantId).Page(page).Limit(limit).Skip(skip).AsTree(asTree).SkipChildren(skipChildren).LimitChildren(limitChildren).MaxTreeDepth(maxTreeDepth).UrlId(urlId).UserId(userId).AnonUserId(anonUserId).ContextUserId(contextUserId).HashTag(hashTag).ParentId(parentId).Direction(direction).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetComments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetComments` : GetComments200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetComments`: %v\n", resp)
}
[inline-code-end]

---