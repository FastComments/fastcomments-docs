### שימוש ב-APIs מאומתים (DefaultApi)

**חשוב:** חייבים להגדיר את מפתח ה-API ב-ApiClient לפני ביצוע בקשות מאומתות. אם לא תעשו זאת, הבקשות ייכשלו עם שגיאת 401.

```ruby
require 'fastcomments'

# צור וקבע תצורה ל-ApiClient
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# דרוש: הגדר את מפתח ה-API שלך (השג אותו מלוח המחוונים של FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# צור את מופע ה-API עם הלקוח שהוגדר
api = FastCommentsClient::DefaultApi.new(api_client)

# עכשיו אפשר לבצע קריאות API מאומתות
begin
  # דוגמה: הוספת משתמש SSO
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
  # - 401: מפתח ה-API חסר או לא תקין
  # - 400: אימות הבקשה נכשל
end
```

### שימוש ב-APIs ציבוריים (PublicApi)

נקודות קצה ציבוריות לא דורשות אימות:

```ruby
require 'fastcomments'

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

### שימוש ב-APIs למודרציה (ModerationApi)

שיטות המודרציה מספקות את לוח הבקרה של המודרטור. שלח אסימון `sso` כך שהבקשה תתבצע בשם מודרטור שאומת באמצעות SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # דוגמה: הצגת תגובות בתור המודרציה
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### בעיות נפוצות

1. **שגיאת 401 "missing-api-key"**: ודא שהגדרת את `config.api_key['x-api-key'] = 'YOUR_KEY'` לפני יצירת האינסטנס של DefaultApi.
2. **מחלקת API שגויה**: השתמש ב-`DefaultApi` לבקשות מאומתות בצד השרת, ב-`PublicApi` לבקשות צד-לקוח/ציבוריות, וב-`ModerationApi` לבקשות עבור לוח הבקרה של המודרטור.
3. **מפתח API null**: ה-SDK ידלג בשקט על אימות אם מפתח ה-API הוא null, מה שיוביל לשגיאות 401.