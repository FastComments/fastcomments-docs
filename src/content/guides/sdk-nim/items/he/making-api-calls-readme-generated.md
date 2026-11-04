כל שיטות ה-API ב-SDK הזה מחזירות tuples של `(Option[ResponseType], Response)`. האלמנט הראשון מכיל את התגובה המפורסרת אם הפעולה הצליחה, והאלמנט השני הוא תגובת ה-HTTP הגולמית.

פרמטרים נדרשים וגוף הבקשה מועברים לפי מיקום (positionally). שאר הפרמטרים האופציונליים נאספים לאובייקט יחיד `Api<Operation>Options`, שהוא הארגומנט האחרון. פעולות ללא פרמטרים אופציונליים לא מקבלות אובייקט אפשרויות.

### דוגמה: שליפת תגובות

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