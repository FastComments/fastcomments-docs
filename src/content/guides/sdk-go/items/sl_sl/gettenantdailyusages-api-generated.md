## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| yearNumber | number | query | Ne |  |
| monthNumber | number | query | Ne |  |
| dayNumber | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_daily_usages_response.go)

## Primer

[inline-code-attrs-start title = 'Primer GetTenantDailyUsages'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	yearNumber := float64(1.2) // float64 |  (izbirno)
	monthNumber := float64(1.2) // float64 |  (izbirno)
	dayNumber := float64(1.2) // float64 |  (izbirno)
	skip := float64(1.2) // float64 |  (izbirno)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantDailyUsages(context.Background()).TenantId(tenantId).YearNumber(yearNumber).MonthNumber(monthNumber).DayNumber(dayNumber).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantDailyUsages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odgovor iz `GetTenantDailyUsages`: GetTenantDailyUsagesResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantDailyUsages`: %v\n", resp)
}
[inline-code-end]