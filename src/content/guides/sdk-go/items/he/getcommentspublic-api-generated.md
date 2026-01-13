req
tenantId
urlId

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| page | integer | query | לא |  |
| direction | string | query | לא |  |
| sso | string | query | לא |  |
| skip | integer | query | לא |  |
| skipChildren | integer | query | לא |  |
| limit | integer | query | לא |  |
| limitChildren | integer | query | לא |  |
| countChildren | boolean | query | לא |  |
| fetchPageForCommentId | string | query | לא |  |
| includeConfig | boolean | query | לא |  |
| countAll | boolean | query | לא |  |
| includei10n | boolean | query | לא |  |
| locale | string | query | לא |  |
| modules | string | query | לא |  |
| isCrawler | boolean | query | לא |  |
| includeNotificationCount | boolean | query | לא |  |
| asTree | boolean | query | לא |  |
| maxTreeDepth | integer | query | לא |  |
| useFullTranslationIds | boolean | query | לא |  |
| parentId | string | query | לא |  |
| searchText | string | query | לא |  |
| hashTags | array | query | לא |  |
| userId | string | query | לא |  |
| customConfigStr | string | query | לא |  |
| afterCommentId | string | query | לא |  |
| beforeCommentId | string | query | לא |  |

## תגובה

מחזיר: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-GetCommentsPublic'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (אופציונלי)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (אופציונלי)
	sso := "sso_example" // string |  (אופציונלי)
	skip := int32(56) // int32 |  (אופציונלי)
	skipChildren := int32(56) // int32 |  (אופציונלי)
	limit := int32(56) // int32 |  (אופציונלי)
	limitChildren := int32(56) // int32 |  (אופציונלי)
	countChildren := true // bool |  (אופציונלי)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (אופציונלי)
	includeConfig := true // bool |  (אופציונלי)
	countAll := true // bool |  (אופציונלי)
	includei10n := true // bool |  (אופציונלי)
	locale := "locale_example" // string |  (אופציונלי)
	modules := "modules_example" // string |  (אופציונלי)
	isCrawler := true // bool |  (אופציונלי)
	includeNotificationCount := true // bool |  (אופציונלי)
	asTree := true // bool |  (אופציונלי)
	maxTreeDepth := int32(56) // int32 |  (אופציונלי)
	useFullTranslationIds := true // bool |  (אופציונלי)
	parentId := "parentId_example" // string |  (אופציונלי)
	searchText := "searchText_example" // string |  (אופציונלי)
	hashTags := []string{"Inner_example"} // []string |  (אופציונלי)
	userId := "userId_example" // string |  (אופציונלי)
	customConfigStr := "customConfigStr_example" // string |  (אופציונלי)
	afterCommentId := "afterCommentId_example" // string |  (אופציונלי)
	beforeCommentId := "beforeCommentId_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ-`GetCommentsPublic`: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]