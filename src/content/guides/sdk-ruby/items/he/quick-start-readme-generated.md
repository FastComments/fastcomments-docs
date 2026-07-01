### שימוש ב‑APIs מאומתים (DefaultApi)

**חשוב:** עליך להגדיר את מפתח ה‑API שלך ב‑ApiClient לפני ביצוע בקשות מאומתות. אם לא תעשה זאת, הבקשות יכשלו עם שגיאת 401.

```ruby
require 'fastcomments'

# צור וקונפיגור את לקוח ה‑API
config = FastCommentsClient::Configuration.new
api_client = FastCommentsClient::ApiClient.new(config)

# דרוש: הגדר את מפתח ה‑API שלך (קבל זאת מלוח המחוונים של FastComments)
config.api_key['x-api-key'] = 'YOUR_API_KEY_HERE'

# צור את מופע ה‑API עם הלקוח המוגדר
api = FastCommentsClient::DefaultApi.new(api_client)

# עכשיו אתה יכול לבצע קריאות API מאומתות
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
  # - 401: מפתח ה‑API חסר או אינו תקין
  # - 400: האימות של הבקשה נכשל
end
```

### שימוש ב‑APIs ציבוריים (PublicApi)

נקודות קצה ציבוריות אינן דורשות אימות:

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

### שימוש ב‑APIs מודרציה (ModerationApi)

שיטות המודרציה מקנות כוח ללוח הבקרה של המודרטור. העבר אסימון `sso` כך שהבקשה תתבצע בשם מודרטור מאומת ב‑SSO:

```ruby
require 'fastcomments'

moderation_api = FastCommentsClient::ModerationApi.new

begin
  # דוגמה: רשימת תגובות בתור המודרציה
  response = moderation_api.get_api_comments(
    sso: 'YOUR_MODERATOR_SSO_TOKEN'
  )
  puts response
rescue FastCommentsClient::ApiError => e
  puts e.message
end
```

### בעיות נפוצות

1. **שגיאת 401 "missing-api-key"**: ודא שהגדרת `config.api_key['x-api-key'] = 'YOUR_KEY'` לפני יצירת מופע DefaultApi.
2. **מחלקת API שגויה**: השתמש ב‑`DefaultApi` עבור בקשות מאומתות בצד השרת, ב‑`PublicApi` עבור בקשות בצד הלקוח/ציבוריות, וב‑`ModerationApi` עבור בקשות ללוח הבקרה של המודרטור.
3. **מפתח API ריק**: ה‑SDK ידלג בשקט על אימות אם מפתח ה‑API ריק, מה שיוביל לשגיאות 401.