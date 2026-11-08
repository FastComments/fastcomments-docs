```bash
go get github.com/fastcomments/fastcomments-go
```

### API क्लाइंट का उपयोग करना

#### Public API (बिना प्रमाणीकरण)

PublicAPI सार्वजनिक एंडपॉइंट्स तक बिना प्रमाणीकरण के एक्सेस की अनुमति देता है:

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

    // PublicAPI का उपयोग करके कमेंट्स प्राप्त करें
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

#### Default API (API Key आवश्यक)

DefaultAPI आपकी API key का उपयोग करके प्रमाणीकरण की आवश्यकता रखता है:

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

    // API key के साथ प्रमाणित context बनाएं
    auth := context.WithValue(
        context.Background(),
        client.ContextAPIKeys,
        map[string]client.APIKey{
            "api_key": {Key: "your-api-key-here"},
        },
    )

    // प्रमाणित DefaultAPI का उपयोग करके कमेंट्स प्राप्त करें
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

#### Moderation API (मॉडरेटर डैशबोर्ड)

ModerationAPI लाइव और तेज़ मॉडरेशन APIs का एक विस्तृत सूट प्रदान करता है। सभी मॉडरेशन मेथड्स एक `sso` पैरामीटर स्वीकार करते हैं और SSO या FastComments.com सेशन कुकी के माध्यम से प्रमाणीकरण कर सकते हैं:

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

    // ModerationAPI का उपयोग करके मॉडरेशन के लिए कमेंट्स सूचीबद्ध करें
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