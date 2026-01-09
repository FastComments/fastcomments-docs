req
tenantId
urlId

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nee |  |
| direction | string | query | Nee |  |
| sso | string | query | Nee |  |
| skip | integer | query | Nee |  |
| skipChildren | integer | query | Nee |  |
| limit | integer | query | Nee |  |
| limitChildren | integer | query | Nee |  |
| countChildren | boolean | query | Nee |  |
| fetchPageForCommentId | string | query | Nee |  |
| includeConfig | boolean | query | Nee |  |
| countAll | boolean | query | Nee |  |
| includei10n | boolean | query | Nee |  |
| locale | string | query | Nee |  |
| modules | string | query | Nee |  |
| isCrawler | boolean | query | Nee |  |
| includeNotificationCount | boolean | query | Nee |  |
| asTree | boolean | query | Nee |  |
| maxTreeDepth | integer | query | Nee |  |
| useFullTranslationIds | boolean | query | Nee |  |
| parentId | string | query | Nee |  |
| searchText | string | query | Nee |  |
| hashTags | array | query | Nee |  |
| userId | string | query | Nee |  |
| customConfigStr | string | query | Nee |  |
| afterCommentId | string | query | Nee |  |
| beforeCommentId | string | query | Nee |  |

## Antwoord

Geeft terug: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## Voorbeeld

[inline-code-attrs-start title = 'GetCommentsPublic Voorbeeld'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (optioneel)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (optioneel)
	sso := "sso_example" // string |  (optioneel)
	skip := int32(56) // int32 |  (optioneel)
	skipChildren := int32(56) // int32 |  (optioneel)
	limit := int32(56) // int32 |  (optioneel)
	limitChildren := int32(56) // int32 |  (optioneel)
	countChildren := true // bool |  (optioneel)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (optioneel)
	includeConfig := true // bool |  (optioneel)
	countAll := true // bool |  (optioneel)
	includei10n := true // bool |  (optioneel)
	locale := "locale_example" // string |  (optioneel)
	modules := "modules_example" // string |  (optioneel)
	isCrawler := true // bool |  (optioneel)
	includeNotificationCount := true // bool |  (optioneel)
	asTree := true // bool |  (optioneel)
	maxTreeDepth := int32(56) // int32 |  (optioneel)
	useFullTranslationIds := true // bool |  (optioneel)
	parentId := "parentId_example" // string |  (optioneel)
	searchText := "searchText_example" // string |  (optioneel)
	hashTags := []string{"Inner_example"} // []string |  (optioneel)
	userId := "userId_example" // string |  (optioneel)
	customConfigStr := "customConfigStr_example" // string |  (optioneel)
	afterCommentId := "afterCommentId_example" // string |  (optioneel)
	beforeCommentId := "beforeCommentId_example" // string |  (optioneel)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// antwoord van `GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]