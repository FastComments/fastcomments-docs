```bash
go get github.com/fastcomments/fastcomments-go
```

### Χρήση του Πελάτη API

#### Δημόσιο API (Χωρίς Επαλήθευση)

Το PublicAPI επιτρέπει μη πιστοποιημένη πρόσβαση σε δημόσια endpoints:

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

#### Προεπιλεγμένο API (Απαιτεί Κλειδί API)

Το DefaultAPI απαιτεί έλεγχο ταυτότητας χρησιμοποιώντας το κλειδί API σας:

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

    // Δημιουργία πιστοποιημένου context με κλειδί API
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // Λήψη σχολίων χρησιμοποιώντας το πιστοποιημένο DefaultAPI
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

#### API Συμμόρφωσης (Πίνακας Διαχειριστή)

Το ModerationAPI παρέχει μια εκτενή σειρά ζωντανών και γρήγορων APIs συμμόρφωσης. Όλες οι μέθοδοι συμμόρφωσης δέχονται μια παράμετρο `sso` και μπορούν να πιστοποιηθούν μέσω SSO ή μέσω cookie συνεδρίας FastComments.com:

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

    // Κατάλογος σχολίων για συμμόρφωση χρησιμοποιώντας το ModerationAPI
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