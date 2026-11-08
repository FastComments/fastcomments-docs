### प्रमाणित APIs (DefaultApi) का उपयोग करना

**महत्वपूर्ण:** प्रमाणित अनुरोध करने से पहले आपको Configuration पर अपनी API key सेट करनी होगी। यदि आप ऐसा नहीं करते, तो अनुरोध 401 त्रुटि के साथ विफल हो जाएंगे।

```python
from client import ApiClient, Configuration, DefaultApi
from client.models import CreateAPISSOUserData

# API क्लाइंट बनाएं और कॉन्फ़िगर करें
config = Configuration()
config.host = "https://fastcomments.com"

# आवश्यक: अपनी API key सेट करें (यह अपने FastComments डैशबोर्ड से प्राप्त करें)
config.api_key = {"api_key": "YOUR_API_KEY_HERE"}

# कॉन्फ़िगर किए गए क्लाइंट के साथ API इंस्टेंस बनाएं
api_client = ApiClient(configuration=config)
api = DefaultApi(api_client)

# अब आप प्रमाणित API कॉल कर सकते हैं
try:
    # उदाहरण: एक SSO उपयोगकर्ता जोड़ें
    user_data = CreateAPISSOUserData(
        id="user-123",
        email="user@example.com",
        display_name="John Doe"
    )

    response = api.add_sso_user("YOUR_TENANT_ID", user_data)
    print(f"User created: {response}")

except Exception as e:
    print(f"Error: {e}")
    # सामान्य त्रुटियाँ:
    # - 401: API key गायब या अमान्य है
    # - 400: अनुरोध सत्यापन विफल रहा
```

### सार्वजनिक APIs (PublicApi) का उपयोग करना

सार्वजनिक एंडपॉइंट्स को प्रमाणीकरण की आवश्यकता नहीं होती:

```python
from client import ApiClient, Configuration, PublicApi

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
public_api = PublicApi(api_client)

try:
    response = public_api.get_comments_public("YOUR_TENANT_ID", "page-url-id")
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### मॉडरेशन डैशबोर्ड (ModerationApi) का उपयोग करना

`ModerationApi` मॉडरेटर डैशबोर्ड को संचालित करता है। मेथड्स को एक `sso` टोकन पास करके मॉडरेटर की ओर से कॉल किया जाता है:

```python
from client import ApiClient, Configuration, ModerationApi
from client.api.moderation_api import GetCountOptions

config = Configuration()
config.host = "https://fastcomments.com"

api_client = ApiClient(configuration=config)
moderation_api = ModerationApi(api_client)

try:
    # मॉडरेशन के लिए प्रतीक्षारत टिप्पणियों की गिनती करें
    response = moderation_api.get_count(GetCountOptions(sso="SSO_TOKEN"))
    print(response)
except Exception as e:
    print(f"Error: {e}")
```

### SSO (Single Sign-On) का उपयोग करना

SDK में सुरक्षित SSO टोकन जेनरेट करने के लिए यूटिलिटीज़ शामिल हैं:

```python
from sso import FastCommentsSSO, SecureSSOUserData

# उपयोगकर्ता डेटा बनाएं (id, email, और username आवश्यक हैं)
user_data = SecureSSOUserData(
    id="user-123",
    email="user@example.com",
    username="johndoe",
    avatar="https://example.com/avatar.jpg"
)

# इसे अपनी API secret (HMAC-SHA256) के साथ साइन करें
sso = FastCommentsSSO.new_secure("YOUR_API_SECRET", user_data)

# विजेट या API कॉल को पास करने के लिए SSO टोकन जेनरेट करें
sso_token = sso.create_token()

# इस टोकन का उपयोग अपने फ्रंटएंड में करें या API कॉल को पास करें
print(f"SSO Token: {sso_token}")
```

सरल SSO के लिए (कम सुरक्षित, परीक्षण के लिए):

```python
from sso import FastCommentsSSO, SimpleSSOUserData

user_data = SimpleSSOUserData(
    username="johndoe",
    email="user@example.com"
)

sso = FastCommentsSSO.new_simple(user_data)
sso_token = sso.create_token()
```

### लाइव सब्सक्रिप्शन (PubSub)

`pubsub` मॉड्यूल आपको WebSocket के माध्यम से रीयल-टाइम टिप्पणी घटनाओं (नई टिप्पणियाँ, वोट, संपादन, सूचनाएँ आदि) की सदस्यता लेने की सुविधा देता है, जो FastComments Java SDK के `LiveEventSubscriber` को दर्शाता है। इसे `pubsub` एक्स्ट्रा की आवश्यकता होती है, जो जेनरेट किए गए API क्लाइंट के ऊपर एक WebSocket क्लाइंट जोड़ता है:

```bash
pip install "fastcomments[pubsub] @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0"
```

```python
from pubsub import LiveEventSubscriber

subscriber = LiveEventSubscriber()

def handle_live_event(event):
    print(f"Live event: {event.type}")
    if event.comment:
        print(f"  comment: {event.comment.comment}")

result = subscriber.subscribe_to_changes(
    tenant_id_ws="YOUR_TENANT_ID",
    url_id="page-url-id",
    url_id_ws="page-url-id",
    user_id_ws="a-unique-presence-id",  # जैसे इस सत्र के लिए एक UUID
    handle_live_event=handle_live_event,
    on_connection_status_change=lambda connected, last_event_time: print(
        f"connected={connected}"
    ),
    region=None,  # EU क्षेत्र के लिए "eu" सेट करें
)

# ...बाद में, जब आप अपडेट नहीं चाहते:
result.close()
```

सब्सक्राइबर कनेक्शन को एक बैकग्राउंड डेमन थ्रेड पर चलाता है, जिटर के साथ पारदर्शी रूप से पुनः कनेक्ट होता है, और पुनः कनेक्ट होने पर डिस्कनेक्ट के दौरान छूटी हुई घटनाओं को event-log एंडपॉइंट से प्राप्त करता है। उन टिप्पणियों की घटनाओं को फ़िल्टर करने के लिए एक वैकल्पिक `can_see_comments` कॉलबैक (`List[str] -> Dict[str, str]`, जो वे ids लौटाता है जिन्हें उपयोगकर्ता नहीं देख सकता) पास करें जिन्हें देखने की उपयोगकर्ता को अनुमति नहीं है। `subscribe_to_changes` को एक no-op बनाने के लिए जो `None` लौटाए, `disable_live_commenting=True` सेट करें।

### सामान्य समस्याएँ

1. **401 "missing-api-key" त्रुटि**: सुनिश्चित करें कि आपने DefaultApi इंस्टेंस बनाने से पहले `config.api_key = {"api_key": "YOUR_KEY"}` सेट किया है।
2. **गलत API क्लास**: सर्वर-साइड प्रमाणित अनुरोधों के लिए `DefaultApi`, क्लाइंट-साइड/सार्वजनिक अनुरोधों के लिए `PublicApi`, और मॉडरेटर डैशबोर्ड अनुरोधों के लिए `ModerationApi` का उपयोग करें।
3. **इंपोर्ट त्रुटियाँ**: सुनिश्चित करें कि आप सही मॉड्यूल से इंपोर्ट कर रहे हैं:
   - API क्लाइंट: `from client import ...`
   - SSO यूटिलिटीज़: `from sso import ...`
   - लाइव सब्सक्रिप्शन: `from pubsub import ...` (`pubsub` एक्स्ट्रा की आवश्यकता है)