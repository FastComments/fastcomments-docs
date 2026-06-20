## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| commentId | string | נתיב | כן |  |
| sso | string | שאילתה | לא |  |

## תגובה

מחזיר: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-go/blob/master/client/model_moderation_api_get_logs_response.go)

## דוגמה

[inline-code-attrs-start title = 'דוגמת GetLogs'; type = 'go'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/fastcomments/fastcomments-go/client"
)

func main() {
	commentId := "commentId_example" // string | 
	sso := "sso_example" // string |  (אופציונלי)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ModerationAPI.GetLogs(context.Background(), commentId).Sso(sso).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ModerationAPI.GetLogs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// תגובה מ`GetLogs`: ModerationAPIGetLogsResponse
	fmt.Fprintf(os.Stdout, "Response from `ModerationAPI.GetLogs`: %v\n", resp)
}
[inline-code-end]