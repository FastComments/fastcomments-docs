FastComments מריץ שרת מודל קונטקסט פרוטוקול (MCP) מתארח כך שעוזרי AI ולקוחות סוכניים יכולים לקרוא ישירות ל-API של FastComments. כל כלי שהשרת MCP מציג נוצר אוטומטית ממפרט OpenAPI הציבורי, ולכן כל מה שה-REST API יכול לעשות, לקוח MCP יכול לעשות.

הקצה הוא חסר-מצב (stateless) ומתבסס על HTTP זורם. אין סשן לשמור בחיים ואין מצב בצד השרת לכל לקוח.

### Endpoint

[inline-code-attrs-start title = 'קצה MCP'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp
[inline-code-end]

### Connect with OAuth

כל לקוח MCP שתומך בשרתים מרוחקים עם OAuth (Claude, ChatGPT, Claude Code, Cursor ואחרים) יכול להתחבר לקצה שלמעלה ללא הגדרה מצד FastComments. הלקוח נרשם דרך רישום לקוח דינמי או מזהה את עצמו עם מסמך מטא-נתוני מזהה לקוח, פותח דפדפן כדי שתוכל להתחבר ל-FastComments ולאשר גישה, ומקבל טוקן הקשור לחשבון שבו נכנסת.

המסמכי גילוי נמצאים במיקומים הסטנדרטיים:

[inline-code-attrs-start title = 'גילוי'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/.well-known/oauth-protected-resource/mcp
https://fastcomments.com/.well-known/oauth-authorization-server
[inline-code-end]

המשתמש שלך צריך את הרשאת API Admin על החשבון כדי לאשר חיבור. אם אתה מנהל כמה חשבונות, עבור לחשבון הנכון בלוח הבקרה לפני האישור.

לקוח יכול לבקש את ההיקף `read`, את ההיקף `write`, או את שניהם. לקוח שלא מבקש דבר מקבל את שניהם. כלים שמשנים נתונים אינם מוצעים לטוקן קריאה בלבד.

לוח הבקרה כולל עוזר הגדרה עם קטעי קוד מוכנים להדבקה. פתח **Integrate -> MCP Server**, או בקר ישירות:

[inline-code-attrs-start title = 'דף הגדרה'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/auth/my-account/mcp-setup
[inline-code-end]

### Claude Code

רשום את שרת FastComments עם פקודה אחת, ואז הרץ `/mcp` בתוך סשן כדי להתחבר ולרשום את הכלים הזמינים:

[inline-code-attrs-start title = 'הגדרת Claude Code'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
claude mcp add --transport http fastcomments https://fastcomments.com/mcp
[inline-code-end]

### Cursor and other config-file clients

הוסף את הבלוק הזה לקונפיגורציית שרתי MCP של הלקוח שלך (`mcp.json` עבור Cursor). הלקוח פותח דפדפן כדי להתחבר בפעם הראשונה.

[inline-code-attrs-start title = 'קונפיגורציית לקוח MCP'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
  "mcpServers": {
    "fastcomments": {
      "type": "http",
      "url": "https://fastcomments.com/mcp"
    }
  }
}
[inline-code-end]

### Revoking access

כל חיבור מאושר מופיע תחת **Integrate -> Connected Apps** בלוח הבקרה. ביטול אחד מבטל את כל הטוקנים שהאפליקציה מחזיקה. אפליקציות נרשמות כאשר הן מתחברות ו-FastComments אינו סוקר אותן, ולכן בטל כל מה שאינך מזהה.

### Using the token with the REST API

טוקן הגישה שלקוח MCP מקבל הוא אישור רגיל של API של FastComments. הוא פועל על כל קצה `/api/v1` כטוקן נושא, ולכן אפליקציה שהתחברה דרך MCP יכולה גם לקרוא ישירות ל-REST API:

[inline-code-attrs-start title = 'טוקן נושא'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl -H "Authorization: Bearer fcat_..." https://fastcomments.com/api/v1/comments
[inline-code-end]

השוכר (tenant) נרמז על ידי הטוקן. עדיין ניתן להעביר `tenantId` אך הוא חייב להתאים. בקשות `GET` דורשות את ההיקף `read` וכל השאר דורש את ההיקף `write`.

### Connect with an API key

לקוחות שלא יכולים להשלים התחברות בדפדפן, כגון שרתים ללא ממשק, יכולים לאמת באמצעות מפתח API במקום זאת. העבר `tenantId` ו-`API_KEY` כפרמטרי שאילתה, או ככותרות HTTP `x-tenant-id` ו-`x-api-key` אם הלקוח שלך תומך בכותרות מותאמות.

[inline-code-attrs-start title = 'קצה מפתח API'; type = 'text'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
https://fastcomments.com/mcp?tenantId=YOUR_TENANT_ID&API_KEY=YOUR_API_KEY
[inline-code-end]

דף ההגדרה יוצר את ה-URL הזה עבור כל מפתח API שלך.

### Security

כתובת קצה שמכילה מפתח API היא סוד: אל תדביק אותה בצ'אטים ציבוריים, צילומי מסך או קומיטים. אם מפתח נחשף, החלף אותו בעמוד מפתחות API בלוח הבקרה שלך. טוקני OAuth אינם נושאים סיכון כזה מכיוון שהם קשורים לאפליקציה אחת וניתן לבטל אותם מ-Connected Apps.