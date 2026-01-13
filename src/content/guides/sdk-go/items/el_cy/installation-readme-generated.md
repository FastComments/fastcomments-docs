```bash
go get github.com/fastcomments/fastcomments-go
```

### Χρήση του API Client

#### Public API (Χωρίς Αυθεντικοποίηση)

Το PublicAPI επιτρέπει μη αυθεντικοποιημένη πρόσβαση σε δημόσια σημεία τερματισμού:

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

    // Λήψη σχολίων χρησιμοποιώντας το PublicAPI
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

#### Default API (Απαιτεί Κλειδί API)

Το DefaultAPI απαιτεί αυθεντικοποίηση χρησιμοποιώντας το κλειδί API σας:

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

    // Δημιουργία αυθεντικοποιημένου context με το κλειδί API
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Λήψη σχολίων χρησιμοποιώντας το αυθεντικοποιημένο DefaultAPI
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