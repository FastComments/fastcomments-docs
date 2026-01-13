req
tenantId
urlId

## 매개변수

| 이름 | 타입 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| page | integer | query | 아니요 |  |
| direction | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |
| skip | integer | query | 아니요 |  |
| skipChildren | integer | query | 아니요 |  |
| limit | integer | query | 아니요 |  |
| limitChildren | integer | query | 아니요 |  |
| countChildren | boolean | query | 아니요 |  |
| fetchPageForCommentId | string | query | 아니요 |  |
| includeConfig | boolean | query | 아니요 |  |
| countAll | boolean | query | 아니요 |  |
| includei10n | boolean | query | 아니요 |  |
| locale | string | query | 아니요 |  |
| modules | string | query | 아니요 |  |
| isCrawler | boolean | query | 아니요 |  |
| includeNotificationCount | boolean | query | 아니요 |  |
| asTree | boolean | query | 아니요 |  |
| maxTreeDepth | integer | query | 아니요 |  |
| useFullTranslationIds | boolean | query | 아니요 |  |
| parentId | string | query | 아니요 |  |
| searchText | string | query | 아니요 |  |
| hashTags | array | query | 아니요 |  |
| userId | string | query | 아니요 |  |
| customConfigStr | string | query | 아니요 |  |
| afterCommentId | string | query | 아니요 |  |
| beforeCommentId | string | query | 아니요 |  |

## 응답

반환: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_comments_public_200_response.go)

## 예제

[inline-code-attrs-start title = 'GetCommentsPublic 예제'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	page := int32(56) // int32 |  (선택 사항)
	direction := openapiclient.SortDirections("OF") // SortDirections |  (선택 사항)
	sso := "sso_example" // string |  (선택 사항)
	skip := int32(56) // int32 |  (선택 사항)
	skipChildren := int32(56) // int32 |  (선택 사항)
	limit := int32(56) // int32 |  (선택 사항)
	limitChildren := int32(56) // int32 |  (선택 사항)
	countChildren := true // bool |  (선택 사항)
	fetchPageForCommentId := "fetchPageForCommentId_example" // string |  (선택 사항)
	includeConfig := true // bool |  (선택 사항)
	countAll := true // bool |  (선택 사항)
	includei10n := true // bool |  (선택 사항)
	locale := "locale_example" // string |  (선택 사항)
	modules := "modules_example" // string |  (선택 사항)
	isCrawler := true // bool |  (선택 사항)
	includeNotificationCount := true // bool |  (선택 사항)
	asTree := true // bool |  (선택 사항)
	maxTreeDepth := int32(56) // int32 |  (선택 사항)
	useFullTranslationIds := true // bool |  (선택 사항)
	parentId := "parentId_example" // string |  (선택 사항)
	searchText := "searchText_example" // string |  (선택 사항)
	hashTags := []string{"Inner_example"} // []string |  (선택 사항)
	userId := "userId_example" // string |  (선택 사항)
	customConfigStr := "customConfigStr_example" // string |  (선택 사항)
	afterCommentId := "afterCommentId_example" // string |  (선택 사항)
	beforeCommentId := "beforeCommentId_example" // string |  (선택 사항)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetCommentsPublic(context.Background(), tenantId).UrlId(urlId).Page(page).Direction(direction).Sso(sso).Skip(skip).SkipChildren(skipChildren).Limit(limit).LimitChildren(limitChildren).CountChildren(countChildren).FetchPageForCommentId(fetchPageForCommentId).IncludeConfig(includeConfig).CountAll(countAll).Includei10n(includei10n).Locale(locale).Modules(modules).IsCrawler(isCrawler).IncludeNotificationCount(includeNotificationCount).AsTree(asTree).MaxTreeDepth(maxTreeDepth).UseFullTranslationIds(useFullTranslationIds).ParentId(parentId).SearchText(searchText).HashTags(hashTags).UserId(userId).CustomConfigStr(customConfigStr).AfterCommentId(afterCommentId).BeforeCommentId(beforeCommentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetCommentsPublic``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetCommentsPublic`의 응답: GetCommentsPublic200Response
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetCommentsPublic`: %v\n", resp)
}
[inline-code-end]