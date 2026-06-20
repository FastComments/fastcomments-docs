## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| yearNumber | number | query | Nie |  |
| monthNumber | number | query | Nie |  |
| dayNumber | number | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantDailyUsagesResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_tenant_daily_usages_response.go)

## Przykład

[inline-code-attrs-start title = 'Przykład GetTenantDailyUsages'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	yearNumber := float64(1.2) // float64 |  (opcjonalne)
	monthNumber := float64(1.2) // float64 |  (opcjonalne)
	dayNumber := float64(1.2) // float64 |  (opcjonalne)
	skip := float64(1.2) // float64 |  (opcjonalne)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetTenantDailyUsages(context.Background()).TenantId(tenantId).YearNumber(yearNumber).MonthNumber(monthNumber).DayNumber(dayNumber).Skip(skip).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetTenantDailyUsages``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// odpowiedź z `GetTenantDailyUsages`: GetTenantDailyUsagesResponse
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetTenantDailyUsages`: %v\n", resp)
}
[inline-code-end]