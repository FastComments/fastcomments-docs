### प्रमाणित APIs का उपयोग (DefaultApi)

**महत्वपूर्ण:** प्रमाणित अनुरोध करने से पहले आपको ApiClient पर अपनी API key सेट करनी होगी। यदि आप ऐसा नहीं करते, तो अनुरोध 401 त्रुटि के साथ विफल होंगे।

```ruby
require 'fastcomments'

# API क्लाइंट बनाएं और कॉन्फ़िगर करें
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# आवश्यक: अपनी API key सेट करें (यह FastComments डैशबोर्ड से प्राप्त करें)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# कॉन्फ़िगर किए गए क्लाइंट के साथ API इंस्टेंस बनाएं
api = FastCommentsClient::DefaultApi.new(api_client)

# अब आप प्रमाणित API कॉल कर सकते हैं
begin
  # उदाहरण: एक SSO उपयोगकर्ता जोड़ें
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # सामान्य त्रुटियाँ:
  # - 401: API key गायब या अमान्य है
  # - 400: अनुरोध सत्यापन विफल रहा
end
```

### सार्वजनिक APIs का उपयोग (PublicApi)

सार्वजनिक एंडपॉइंट्स को प्रमाणीकरण की आवश्यकता नहीं है:

```ruby
require 'fastcomments'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    'YOUR_TENANT_ID',
    'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### मॉडरेशन APIs का उपयोग (ModerationApi)

मॉडरेशन मेथड्स मॉडरेटर डैशबोर्ड को संचालित करते हैं। एक `sso` टोकन पास करें ताकि अनुरोध एक SSO-प्रमाणित मॉडरेटर की ओर से किया जाए:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # उदाहरण: मॉडरेशन कतार में टिप्पणियाँ सूचीबद्ध करें
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### सामान्य समस्याएँ

1. **401 "missing-api-key" त्रुटि**: सुनिश्चित करें कि आपने DefaultApi इंस्टेंस बनाने से पहले `config.api_key['x-api-key'] = 'YOUR_KEY'` सेट किया है।
2. **गलत API क्लास**: सर्वर-साइड प्रमाणित अनुरोधों के लिए `DefaultApi`, क्लाइंट-साइड/सार्वजनिक अनुरोधों के लिए `PublicApi`, और मॉडरेटर डैशबोर्ड अनुरोधों के लिए `ModerationApi` का उपयोग करें।
3. **Null API key**: यदि API key null है तो SDK चुपचाप प्रमाणीकरण छोड़ देगा, जिससे 401 त्रुटियाँ होंगी।