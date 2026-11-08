req
tenantId
urlId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| direction | string | query | नहीं |  |
| sso | string | query | नहीं |  |
| skip | integer | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| countChildren | boolean | query | नहीं |  |
| fetchPageForCommentId | string | query | नहीं |  |
| includeConfig | boolean | query | नहीं |  |
| countAll | boolean | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| modules | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |
| includeNotificationCount | boolean | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| useFullTranslationIds | boolean | query | नहीं |  |
| parentId | string | query | नहीं |  |
| searchText | string | query | नहीं |  |
| hashTags | array | query | नहीं |  |
| userId | string | query | नहीं |  |
| customConfigStr | string | query | नहीं |  |
| afterCommentId | string | query | नहीं |  |
| beforeCommentId | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_response_with_presence_public_comment_.go)

## उदाहरण

[inline-code-attrs-start title = 'GetCommentsPublic उदाहरण'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	urlId := "urlId_example" // string | 
	page := int32(56) // int32 |  (optional)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (optional)
	sso := "sso_example" // string |  (optional)
	skip := int32(56) // int32 |  (optional)
	skipChildren := int32(56) // int32 |  (optional)
	limit := int32(56) // int32 |  (optional)
	limitChildren := int32(56) // int32 |  (optional)
	countChildren := true // bool |  (optional)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (optional)
	includeConfig := true // bool |  (optional)
	countAll := true // bool |  (optional)
	includei10n := true // bool |  (optional)
	locale := "locale_example" // string |  (optional)
	modules := "modules_example" // string |  (optional)
	isCrawler := true // bool |  (optional)
	includeNotificationCount := true // bool |  (optional)
	asTree := true // bool |  (optional)
	maxTreeDepth := int32(56) // int32 |  (optional)
	useFullTranslationIds := true // bool |  (optional)
	parentId := "parentId_example" // string |  (optional)
	searchText := "searchText_example" // string |  (optional)
	hashTags := []string{"Inner_example"} // []string |  (optional)
	userId := "userId_example" // string |  (optional)
	customConfigStr := "customConfigStr_example" // string |  (optional)
	afterCommentId := "afterCommentId_example" // string |  (optional)
	beforeCommentId := "beforeCommentId_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetCommentsPublic`: GetCommentsResponseWithPresencePublicComment
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]