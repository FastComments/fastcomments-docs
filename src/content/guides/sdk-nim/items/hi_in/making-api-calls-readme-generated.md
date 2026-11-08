इस SDK की सभी API मेथड्स `(Option[ResponseType], Response)` के टुपल्स लौटाती हैं। पहला एलिमेंट सफल होने पर पार्स किया गया रिस्पॉन्स रखता है, और दूसरा एलिमेंट रॉ HTTP रिस्पॉन्स है।

आवश्यक पैरामीटर और रिक्वेस्ट बॉडी पोज़िशनल रूप से पास किए जाते हैं। शेष वैकल्पिक पैरामीटर एक ही `Api<Operation>Options` ऑब्जेक्ट में संग्रहित होते हैं, जो अंतिम आर्ग्युमेंट होता है। जिन ऑपरेशनों में कोई वैकल्पिक पैरामीटर नहीं होता, वे कोई ऑप्शंस ऑब्जेक्ट नहीं लेते।

### उदाहरण: कमेंट्स प्राप्त करना

```nim
import httpclient
import options
import fastcomments
import fastcomments/apis/api_default

let client = newHttpClient()
client.headers["x-api-key"] = "your-api-key"

let (response, httpResponse) = getComments(
  httpClient = client,
  tenantId = "your-tenant-id",
  options = GetCommentsOptions(
    urlId: "your-url-id",
    direction: SortDirections.DESC
  )
)

if httpResponse.code == Http200:
  if response.isSome:
    let resp = response.get()
    if resp.comments.isSome:
      echo "Found ", resp.comments.get().len, " comments"
```