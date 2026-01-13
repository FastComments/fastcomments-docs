## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| yearNumber | number | query | 否 |  |
| monthNumber | number | query | 否 |  |
| dayNumber | number | query | 否 |  |
| skip | number | query | 否 |  |

## 响应

返回: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_daily_usages_200_response.go)

## 示例

[inline-code-attrs-start title = 'GetTenantDailyUsages 示例'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	yearNumber := float64(1.2) // float64 |  (可选)
	monthNumber := float64(1.2) // float64 |  (可选)
	dayNumber := float64(1.2) // float64 |  (可选)
	skip := float64(1.2) // float64 |  (可选)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantDailyUsages(context.Background()).TenantId(tenantId).YearNumber(yearNumber).MonthNumber(monthNumber).DayNumber(dayNumber).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantDailyUsages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// 来自 `GetTenantDailyUsages` 的响应: GetTenantDailyUsages200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantDailyUsages`: %v\n", resp)
}
[inline-code-end]