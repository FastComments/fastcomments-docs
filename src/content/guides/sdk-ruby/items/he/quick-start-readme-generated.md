### שימוש ב-APIs מאומתים (DefaultApi)

**חשוב:** עליך להגדיר את מפתח ה-API ב-ApiClient לפני ביצוע בקשות מאומתות. אם לא תעשה זאת, הבקשות יחזרו עם שגיאת 401.

```ruby
require 'fastcomments-client'

# צור והגדר את לקוח ה-API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# דרוש: הגדר את מפתח ה-API שלך (קבל אותו מדשבורד FastComments שלך)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# צור את מופע ה-API עם הלקוח שהוגדר
api = FastCommentsClient::DefaultApi.new(api_client)

# עכשיו תוכל לבצע קריאות API מאומתות
begin
  # דוגמה: הוסף משתמש SSO
  user_data = {
    id: 'user-123',
    email: 'user@example.com',
    displayName: 'John Doe'
  }

  response = api.add_sso_user('YOUR_TENANT_ID', user_data)
  puts "User created: #{response}"

rescue FastCommentsClient::ApiError => e
  puts "Error: #{e.response_body}"
  # שגיאות נפוצות:
  # - 401: מפתח ה-API חסר או לא תקף
  # - 400: אימות הבקשה נכשל
end
```

### שימוש ב-APIs ציבוריים (PublicApi)

נקודות קצה ציבוריות אינן דורשות אימות:

```ruby
require 'fastcomments-client'

public_api = FastCommentsClient::PublicApi.new

begin
  response = public_api.get_comments_public(
    tenant_id: 'YOUR_TENANT_ID',
    url_id: 'page-url-id'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### בעיות נפוצות

1. **שגיאת 401 "missing-api-key"**: ודא שאתה מגדיר `config.api_key['x-api-key'] = 'YOUR_KEY'` לפני יצירת מופע DefaultApi.
2. **מחלקת API שגויה**: השתמש ב-`DefaultApi` עבור בקשות מאומתות בצד השרת, וב-`PublicApi` עבור בקשות בצד הלקוח/ציבוריות.
3. **מפתח API ריק**: ה-SDK פשוט ידלג על אימות אם מפתח ה-API הוא null, מה שייגרום לשגיאות 401.