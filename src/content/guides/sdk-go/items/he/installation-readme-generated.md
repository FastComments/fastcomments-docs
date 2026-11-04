```bash
go get github.com/fastcomments/fastcomments-go
```

### שימוש בלקוח ה-API

#### API ציבורי (ללא אימות)

ה-PublicAPI מאפשר גישה לא מאומתת לנקודות קצה ציבוריות:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // קבלת תגובות באמצעות PublicAPI
    response, httpResp, err := apiClient.PublicAPI.GetCommentsPublic(
        context.Background(),
        "your-tenant-id",
    ).UrlId("your-page-url-id").Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```

#### API ברירת מחדל (דורש מפתח API)

ה-DefaultAPI דורש אימות באמצעות מפתח ה-API שלך:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // יצירת הקשר מאומת עם מפתח API
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // קבלת תגובות באמצעות DefaultAPI מאומת
    response, httpResp, err := apiClient.DefaultAPI.GetComments(auth).
        TenantId("your-tenant-id").
        UrlId("your-page-url-id").
        Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```

#### API לניהול (לוח בקרה של מנחה)

ה-ModerationAPI מספק חבילה נרחבת של APIs לניהול חי ומהיר. כל שיטות הניהול מקבלות פרמטר `sso` ויכולות לבצע אימות באמצעות SSO או קוקי סשן של FastComments.com:

```go
package main

import (
    "context"
    "fmt"
    "github.com/fastcomments/fastcomments-go/client"
)

func main() {
    config := client.NewConfiguration()
    apiClient := client.NewAPIClient(config)

    // רשימת תגובות לניהול באמצעות ModerationAPI
    response, httpResp, err := apiClient.ModerationAPI.GetApiComments(
        context.Background(),
    ).Sso("your-sso-token").Execute()

    if err != nil {
        panic(err)
    }

    fmt.Printf("Status: %d\n", httpResp.StatusCode)
    fmt.Printf("Comments: %+v\n", response)
}
```