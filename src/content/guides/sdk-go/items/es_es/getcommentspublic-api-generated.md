req
tenantId
urlId

## Parámetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sí |  |
| urlId | string | query | Sí |  |
| page | integer | query | No |  |
| direction | string | query | No |  |
| sso | string | query | No |  |
| skip | integer | query | No |  |
| skipChildren | integer | query | No |  |
| limit | integer | query | No |  |
| limitChildren | integer | query | No |  |
| countChildren | boolean | query | No |  |
| fetchPageForCommentId | string | query | No |  |
| includeConfig | boolean | query | No |  |
| countAll | boolean | query | No |  |
| includei10n | boolean | query | No |  |
| locale | string | query | No |  |
| modules | string | query | No |  |
| isCrawler | boolean | query | No |  |
| includeNotificationCount | boolean | query | No |  |
| asTree | boolean | query | No |  |
| maxTreeDepth | integer | query | No |  |
| useFullTranslationIds | boolean | query | No |  |
| parentId | string | query | No |  |
| searchText | string | query | No |  |
| hashTags | array | query | No |  |
| userId | string | query | No |  |
| customConfigStr | string | query | No |  |
| afterCommentId | string | query | No |  |
| beforeCommentId | string | query | No |  |

## Response

Devuelve: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	page := int32(56) // int32 |  (opcional)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (opcional)
	sso := "sso_example" // string |  (opcional)
	skip := int32(56) // int32 |  (opcional)
	skipChildren := int32(56) // int32 |  (opcional)
	limit := int32(56) // int32 |  (opcional)
	limitChildren := int32(56) // int32 |  (opcional)
	countChildren := true // bool |  (opcional)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (opcional)
	includeConfig := true // bool |  (opcional)
	countAll := true // bool |  (opcional)
	includei10n := true // bool |  (opcional)
	locale := "locale_example" // string |  (opcional)
	modules := "modules_example" // string |  (opcional)
	isCrawler := true // bool |  (opcional)
	includeNotificationCount := true // bool |  (opcional)
	asTree := true // bool |  (opcional)
	maxTreeDepth := int32(56) // int32 |  (opcional)
	useFullTranslationIds := true // bool |  (opcional)
	parentId := "parentId_example" // string |  (opcional)
	searchText := "searchText_example" // string |  (opcional)
	hashTags := []string{"Inner_example"} // []string |  (opcional)
	userId := "userId_example" // string |  (opcional)
	customConfigStr := "customConfigStr_example" // string |  (opcional)
	afterCommentId := "afterCommentId_example" // string |  (opcional)
	beforeCommentId := "beforeCommentId_example" // string |  (opcional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// respuesta de `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]