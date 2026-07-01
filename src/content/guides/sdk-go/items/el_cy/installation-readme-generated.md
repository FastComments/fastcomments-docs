```bash
go get github.com/fastcomments/fastcomments-go
```

### Χρήση του Πελάτη API

#### Δημόσιο API (Χωρίς Πιστοποίηση)

Το **PublicAPI** επιτρέπει πρόσβαση χωρίς πιστοποίηση στα δημόσια σημεία λήψης:

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

    // Get comments using PublicAPI
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

#### Προεπιλεγμένο API (Απαιτεί Κλειδί API)

Το **DefaultAPI** απαιτεί πιστοποίηση χρησιμοποιώντας το κλειδί API σας:

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

    // Create authenticated context with API key
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Get comments using authenticated DefaultAPI
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

#### Moderation API (Πίνακας Ελεγκτή)

Το **ModerationAPI** προσφέρει ένα εκτενές σύνολο ζωντανών και γρήγορων APIs διαχείρισης. Όλες οι μέθοδοι διαχείρισης δέχονται μια παράμετρο `sso` και μπορούν να πιστοποιηθούν μέσω SSO ή ενός cookie συνεδρίας του FastComments.com:

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

    // List comments for moderation using ModerationAPI
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