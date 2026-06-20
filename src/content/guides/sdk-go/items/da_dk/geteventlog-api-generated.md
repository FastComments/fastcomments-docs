req
tenantId
urlId
userIdWS

## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | sti | Ja |  |
| urlId | string | forespørgsel | Ja |  |
| userIdWS | string | forespørgsel | Ja |  |
| startTime | integer | forespørgsel | Ja |  |
| endTime | integer | forespørgsel | Nej |  |

## Svar

Returnerer: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_get_event_log_response.go)

## Eksempel

[inline-code-attrs-start title = 'GetEventLog-eksempel'; type = 'go'; isFunctional = false; inline-code-attrs-end]
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
	userIdWS := "userIdWS_example" // string | 
	startTime := int64(789) // int64 | 
	endTime := int64(789) // int64 |  (valgfri)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PublicAPI.GetEventLog(context.Background(), tenantId).UrlId(urlId).UserIdWS(userIdWS).StartTime(startTime).EndTime(endTime).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PublicAPI.GetEventLog``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// svar fra `GetEventLog`: GetEventLogResponse
	fmt.Fprintf(os.Stdout, "Response from `PublicAPI.GetEventLog`: %v\n", resp)
}
[inline-code-end]