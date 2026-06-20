```bash
go get github.com/fastcomments/fastcomments-go
```

### שימוש בלקוח ה-API

#### Public API (ללא אימות)

The PublicAPI מאפשרת גישה ללא אימות לנקודות קצה ציבוריות:

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

    // קבל תגובות באמצעות PublicAPI
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

#### Default API (דורש מפתח API)

The DefaultAPI דורש אימות באמצעות מפתח ה-API שלך:

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

    // צור הקשר מאומת עם מפתח ה-API
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // קבל תגובות באמצעות DefaultAPI מאומת
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

#### Moderation API (לוח הבקרה של המודרטור)

The ModerationAPI מפעיל את לוח הבקרה של המודרטור. הוא מספק שיטות לרישום,
ספירה, חיפוש וייצוא תגובות, פעולות פיקוח (הסרה/שחזור,
דגל, קביעת סטטוס לסקירה/ספאם/אישור, הצבעות, פתיחה/סגירה של שרשורים), הרחקות (חסימה מלהגיב, בטל, תקצירי טרום-חסימה, סטטוס והעדפות חסימה, ספירות משתמשים חסומים),
וסמלים ואמון (הענקה/הסרה של תגי כבוד, תגי ידני, קבלת/קביעת גורם אמון, פרופיל פנימי של משתמש). כל שיטות Moderation מקבלות פרמטר `sso` עבור
מודרטורים המאומתים באמצעות SSO:

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

    // רשימת תגובות למודרציה באמצעות ModerationAPI
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