```bash
go get github.com/fastcomments/fastcomments-go
```

### Χρήση του πελάτη API

#### Δημόσιο API (Χωρίς Αυθεντικοποίηση)

Η PublicAPI επιτρέπει μη αυθεντικοποιημένη πρόσβαση σε δημόσια endpoints:

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

    // Λήψη σχολίων με χρήση της PublicAPI
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

Η DefaultAPI απαιτεί αυθεντικοποίηση χρησιμοποιώντας το κλειδί API σας:

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

    // Λήψη σχολίων χρησιμοποιώντας την αυθεντικοποιημένη DefaultAPI
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

Η ModerationAPI τροφοδοτεί τον πίνακα ελέγχου των συντονιστών. Παρέχει μεθόδους για την καταγραφή, μέτρηση, αναζήτηση και εξαγωγή σχολίων, ενέργειες εποπτείας (αφαίρεση/ανάκτηση, σήμανση, ορισμός κατάστασης ανασκόπησης/spam/έγκρισης, ψήφοι, επαναφορά/κλείσιμο νήματος), απαγορεύσεις (απαγόρευση σχολιασμού, αναίρεση, προ-συνοπτικές πληροφορίες περί απαγόρευσης, κατάσταση και προτιμήσεις απαγόρευσης, αριθμοί αποκλεισμένων χρηστών), και διακριτικά & εμπιστοσύνη (απονομή/αφαίρεση διακριτικών, χειροκίνητα διακριτικά, λήψη/ρύθμιση παράγοντα εμπιστοσύνης, εσωτερικό προφίλ χρήστη). Όλες οι μέθοδοι Moderation δέχονται παράμετρο `sso` για συντονιστές αυθεντικοποιημένους μέσω SSO:

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

    // Λίστα σχολίων για εποπτεία χρησιμοποιώντας την ModerationAPI
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