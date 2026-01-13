## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| yearNumber | number | query | いいえ |  |
| monthNumber | number | query | いいえ |  |
| dayNumber | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

返却: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_daily_usages_200_response.go)

## 例

[inline-code-attrs-start title = 'GetTenantDailyUsages の例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	yearNumber := float64(1.2) // float64 |  (任意)
	monthNumber := float64(1.2) // float64 |  (任意)
	dayNumber := float64(1.2) // float64 |  (任意)
	skip := float64(1.2) // float64 |  (任意)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantDailyUsages(context.Background()).TenantId(tenantId).YearNumber(yearNumber).MonthNumber(monthNumber).DayNumber(dayNumber).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantDailyUsages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetTenantDailyUsages` からのレスポンス: GetTenantDailyUsages200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantDailyUsages`: %v\n", resp)
}
[inline-code-end]

---