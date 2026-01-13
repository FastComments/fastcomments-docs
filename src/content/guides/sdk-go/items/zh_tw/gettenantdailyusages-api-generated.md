## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| yearNumber | number | query | 否 |  |
| monthNumber | number | query | 否 |  |
| dayNumber | number | query | 否 |  |
| skip | number | query | 否 |  |

## 回應

回傳: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_daily_usages_200_response.go)

## 範例

[inline-code-attrs-start title = 'GetTenantDailyUsages 範例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	yearNumber := float64(1.2) // float64 |  （可選）
	monthNumber := float64(1.2) // float64 |  （可選）
	dayNumber := float64(1.2) // float64 |  （可選）
	skip := float64(1.2) // float64 |  （可選）

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantDailyUsages(context.Background()).TenantId(tenantId).YearNumber(yearNumber).MonthNumber(monthNumber).DayNumber(dayNumber).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantDailyUsages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// `GetTenantDailyUsages` 的回應: GetTenantDailyUsages200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantDailyUsages`: %v\n", resp)
}
[inline-code-end]

---