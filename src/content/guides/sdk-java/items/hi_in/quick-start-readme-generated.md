### प्रमाणित APIs (DefaultApi) का उपयोग करना

**महत्वपूर्ण:** प्रमाणित अनुरोध करने से पहले आपको ApiClient पर अपनी API key सेट करनी होगी। यदि आप ऐसा नहीं करते, तो अनुरोध 401 त्रुटि के साथ विफल हो जाएंगे।

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // API क्लाइंट बनाएं और कॉन्फ़िगर करें
        ApiClient apiClient = new ApiClient();

        // आवश्यक: अपनी API key सेट करें (यह अपने FastComments डैशबोर्ड से प्राप्त करें)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // कॉन्फ़िगर किए गए क्लाइंट के साथ API इंस्टेंस बनाएं
        DefaultApi api = new DefaultApi(apiClient);

        // अब आप प्रमाणित API कॉल कर सकते हैं
        try {
            // उदाहरण: एक SSO उपयोगकर्ता जोड़ें
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // सामान्य त्रुटियाँ:
            // - 401: API key गायब या अमान्य है
            // - 400: अनुरोध सत्यापन विफल रहा
        }
    }
}
```

### सार्वजनिक APIs (PublicApi) का उपयोग करना

सार्वजनिक एंडपॉइंट्स को प्रमाणीकरण की आवश्यकता नहीं है:

```java
import com.fastcomments.api.PublicApi;
import com.fastcomments.invoker.ApiException;

PublicApi publicApi = new PublicApi();

try {
    var response = publicApi.getCommentsPublic("YOUR_TENANT_ID", "page-url-id")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### मॉडरेशन APIs (ModerationApi) का उपयोग करना

`ModerationApi` मॉडरेटर डैशबोर्ड को संचालित करता है। प्रत्येक मेथड एक `sso` पैरामीटर स्वीकार करता है जो उस SSO-प्रमाणित मॉडरेटर की पहचान करता है जिसकी ओर से अनुरोध किया जाता है:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // मॉडरेशन की प्रतीक्षा कर रही टिप्पणियों की सूची बनाएं
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### सामान्य समस्याएँ

1. **401 "missing-api-key" त्रुटि**: सुनिश्चित करें कि आप DefaultApi इंस्टेंस बनाने से पहले `apiClient.setApiKey("YOUR_KEY")` कॉल करते हैं।
2. **गलत API क्लास**: सर्वर-साइड प्रमाणित अनुरोधों के लिए `DefaultApi` और क्लाइंट-साइड/सार्वजनिक अनुरोधों के लिए `PublicApi` का उपयोग करें।
3. **Null API key**: यदि API key null है तो SDK चुपचाप प्रमाणीकरण छोड़ देगा, जिससे 401 त्रुटियाँ होंगी।