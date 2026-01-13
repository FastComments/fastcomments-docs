påkrævet
tenantId
urlId

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nej |  |
| direction | string | query | Nej |  |
| sso | string | query | Nej |  |
| skip | integer | query | Nej |  |
| skipChildren | integer | query | Nej |  |
| limit | integer | query | Nej |  |
| limitChildren | integer | query | Nej |  |
| countChildren | boolean | query | Nej |  |
| fetchPageForCommentId | string | query | Nej |  |
| includeConfig | boolean | query | Nej |  |
| countAll | boolean | query | Nej |  |
| includei10n | boolean | query | Nej |  |
| locale | string | query | Nej |  |
| modules | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |
| includeNotificationCount | boolean | query | Nej |  |
| asTree | boolean | query | Nej |  |
| maxTreeDepth | integer | query | Nej |  |
| useFullTranslationIds | boolean | query | Nej |  |
| parentId | string | query | Nej |  |
| searchText | string | query | Nej |  |
| hashTags | array | query | Nej |  |
| userId | string | query | Nej |  |
| customConfigStr | string | query | Nej |  |
| afterCommentId | string | query | Nej |  |
| beforeCommentId | string | query | Nej |  |

## Svar

Returnerer: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (valgfri)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (valgfri)
	sso := "sso_example" // string |  (valgfri)
	skip := int32(56) // int32 |  (valgfri)
	skipChildren := int32(56) // int32 |  (valgfri)
	limit := int32(56) // int32 |  (valgfri)
	limitChildren := int32(56) // int32 |  (valgfri)
	countChildren := true // bool |  (valgfri)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (valgfri)
	includeConfig := true // bool |  (valgfri)
	countAll := true // bool |  (valgfri)
	includei10n := true // bool |  (valgfri)
	locale := "locale_example" // string |  (valgfri)
	modules := "modules_example" // string |  (valgfri)
	isCrawler := true // bool |  (valgfri)
	includeNotificationCount := true // bool |  (valgfri)
	asTree := true // bool |  (valgfri)
	maxTreeDepth := int32(56) // int32 |  (valgfri)
	useFullTranslationIds := true // bool |  (valgfri)
	parentId := "parentId_example" // string |  (valgfri)
	searchText := "searchText_example" // string |  (valgfri)
	hashTags := []string{"Inner_example"} // []string |  (valgfri)
	userId := "userId_example" // string |  (valgfri)
	customConfigStr := "customConfigStr_example" // string |  (valgfri)
	afterCommentId := "afterCommentId_example" // string |  (valgfri)
	beforeCommentId := "beforeCommentId_example" // string |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]