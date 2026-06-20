```bash
go get github.com/fastcomments/fastcomments-go
```

### Χρήση του πελάτη API

#### Δημόσιο API (Χωρίς Πιστοποίηση)

Το PublicAPI επιτρέπει μη αυθεντικοποιημένη πρόσβαση σε δημόσια endpoints:

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

Το DefaultAPI απαιτεί αυθεντικοποίηση με το κλειδί API σας:

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

    // Δημιουργία αυθεντικοποιημένου context με κλειδί API
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

#### Moderation API (Πίνακας Ελέγχου Συντονιστή)

Το ModerationAPI τροφοδοτεί τον πίνακα ελέγχου των συντονιστών. Παρέχει μεθόδους για την καταγραφή, μέτρηση, αναζήτηση και εξαγωγή σχολίων, ενέργειες εποπτείας (αφαίρεση/επαναφορά, σήμανση, ορισμός κατάστασης για αναθεώρηση/spam/έγκριση, ψήφοι, επανάνοιγμα/κλείσιμο νημάτων), αποκλεισμούς (αποκλεισμός από σχολιασμό, αναίρεση, περιλήψεις πριν από αποκλεισμό, κατάσταση αποκλεισμού και προτιμήσεις, αριθμοί αποκλεισμένων χρηστών), και διακριτικά & εμπιστοσύνη (απονομή/αφαίρεση διακριτικών, χειροκίνητα διακριτικά, λήψη/ορισμός παράγοντα εμπιστοσύνης, εσωτερικό προφίλ χρήστη). Όλες οι μέθοδοι Moderation δέχονται παράμετρο `sso` για συντονιστές με SSO αυθεντικοποίηση:

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

    // Λίστα σχολίων για εποπτεία με χρήση του ModerationAPI
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