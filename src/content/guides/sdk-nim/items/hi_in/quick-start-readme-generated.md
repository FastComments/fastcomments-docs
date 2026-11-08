### प्रमाणित APIs का उपयोग (DefaultAPI)

**महत्वपूर्ण:** प्रमाणित endpoints के लिए आपकी API key को `x-api-key` header के रूप में सेट करना आवश्यक है।

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_default
import fastcomments/models/model_comment_data

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

# प्रमाणित API कॉल करें।
# आवश्यक पैरामीटर (और request body) positional होते हैं; वैकल्पिक
# पैरामीटर operation के options object के माध्यम से पास किए जाते हैं।
let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### सार्वजनिक APIs का उपयोग (PublicAPI)

सार्वजनिक endpoints के लिए प्रमाणीकरण की आवश्यकता नहीं है:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_public

let client = newHttpClient()

# सार्वजनिक API कॉल करें।
# tenantId और urlId आवश्यक हैं (positional); बाकी सब वैकल्पिक है।
let (response, httpResponse) = getCommentsPublic(
  httpClient = client,
  tenantId = "your-tenant-id",
  urlId = "your-url-id",
  options = GetCommentsPublicOptions(
    direction: SortDirections.DESC
  )
)

if response.isSome:
  let resp = response.get()
  if resp.comments.isSome:
    echo "Found ", resp.comments.get().len, " comments"
```

### मॉडरेशन APIs का उपयोग (ModerationAPI)

मॉडरेशन endpoints मॉडरेटर डैशबोर्ड को संचालित करते हैं और कार्यरत मॉडरेटर के लिए SSO token से प्रमाणित होते हैं:

```nim
import httpclient
import fastcomments
import fastcomments/apis/api_moderation

let client = newHttpClient()

# मॉडरेशन डैशबोर्ड में कमेंट्स की सूची बनाएं।
# इस operation में कोई आवश्यक पैरामीटर नहीं है, इसलिए सब कुछ वैकल्पिक है।
let (response, httpResponse) = getApiComments(
  httpClient = client,
  options = GetApiCommentsOptions(
    count: 30,
    tenantId: "your-tenant-id",
    sso: "your-sso-token"
  )
)

if response.isSome:
  let resp = response.get()
  echo "Found ", resp.comments.len, " comments"
```

### सामान्य समस्याएँ

1. **401 प्रमाणीकरण त्रुटि**: DefaultAPI requests करने से पहले अपने HttpClient पर `x-api-key` header सेट करना सुनिश्चित करें: `client.headers["x-api-key"] = "your-api-key"`
2. **गलत API class**: सर्वर-साइड प्रमाणित requests के लिए `api_default`, क्लाइंट-साइड/सार्वजनिक requests के लिए `api_public`, और मॉडरेटर डैशबोर्ड requests के लिए `api_moderation` का उपयोग करें।