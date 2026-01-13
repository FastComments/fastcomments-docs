req
tenantId
urlId

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| page | integer | query | Non |  |
| direction | string | query | Non |  |
| sso | string | query | Non |  |
| skip | integer | query | Non |  |
| skipChildren | integer | query | Non |  |
| limit | integer | query | Non |  |
| limitChildren | integer | query | Non |  |
| countChildren | boolean | query | Non |  |
| fetchPageForCommentId | string | query | Non |  |
| includeConfig | boolean | query | Non |  |
| countAll | boolean | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| modules | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeNotificationCount | boolean | query | Non |  |
| asTree | boolean | query | Non |  |
| maxTreeDepth | integer | query | Non |  |
| useFullTranslationIds | boolean | query | Non |  |
| parentId | string | query | Non |  |
| searchText | string | query | Non |  |
| hashTags | array | query | Non |  |
| userId | string | query | Non |  |
| customConfigStr | string | query | Non |  |
| afterCommentId | string | query | Non |  |
| beforeCommentId | string | query | Non |  |

## Réponse

Retourne : [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Exemple

[inline-code-attrs-start title = 'Exemple de GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (facultatif)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (facultatif)
	sso := "sso_example" // string |  (facultatif)
	skip := int32(56) // int32 |  (facultatif)
	skipChildren := int32(56) // int32 |  (facultatif)
	limit := int32(56) // int32 |  (facultatif)
	limitChildren := int32(56) // int32 |  (facultatif)
	countChildren := true // bool |  (facultatif)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (facultatif)
	includeConfig := true // bool |  (facultatif)
	countAll := true // bool |  (facultatif)
	includei10n := true // bool |  (facultatif)
	locale := "locale_example" // string |  (facultatif)
	modules := "modules_example" // string |  (facultatif)
	isCrawler := true // bool |  (facultatif)
	includeNotificationCount := true // bool |  (facultatif)
	asTree := true // bool |  (facultatif)
	maxTreeDepth := int32(56) // int32 |  (facultatif)
	useFullTranslationIds := true // bool |  (facultatif)
	parentId := "parentId_example" // string |  (facultatif)
	searchText := "searchText_example" // string |  (facultatif)
	hashTags := []string{"Inner_example"} // []string |  (facultatif)
	userId := "userId_example" // string |  (facultatif)
	customConfigStr := "customConfigStr_example" // string |  (facultatif)
	afterCommentId := "afterCommentId_example" // string |  (facultatif)
	beforeCommentId := "beforeCommentId_example" // string |  (facultatif)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// réponse de `GetCommentsPublic` : GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]