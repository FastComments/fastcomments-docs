רשימת דפים עבור tenant. משמשת על ידי לקוח שולחן העבודה של FChat כדי למלא את רשימת החדרים שלו.
דורש ש-`enableFChat` יהיה true בקונפיגורציית ה-custom שנפתרה עבור כל דף.
דפים שדורשים SSO מסוננים לפי גישת הקבוצות של המשתמש המבקש.

## פרמטרים

| שם | סוג | מיקום | דרוש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| cursor | string | query | לא | מצביע דפדוף אטום שהוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | לא | 1..200, ברירת מחדל 50 |
| q | string | query | לא | מסנן אופציונלי על תחילת הכותרת, שאינו רגיש להבדל בין אותיות גדולות וקטנות. |
| sortBy | string | query | לא | סדר המיון. `updatedAt` (ברירת מחדל, החדשים ביותר תחילה), `commentCount` (הכי הרבה תגובות תחילה), או `title` (אלפביתי). |
| hasComments | boolean | query | לא | אם true, החזר רק דפים עם לפחות תגובה אחת. |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## דוגמה

[inline-code-attrs-start title = 'get_pages_public דוגמה'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
opts = {
  cursor: 'cursor_example', # מחרוזת | מצביע דפדוף אטום שהוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`.
  limit: 56, # מספר שלם | 1..200, ברירת מחדל 50
  q: 'q_example', # מחרוזת | מסנן אופציונלי על תחילת הכותרת, שאינו רגיש להבדל בין אותיות גדולות וקטנות.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | סדר המיון. `updatedAt` (ברירת מחדל, החדשים ביותר תחילה), `commentCount` (הכי הרבה תגובות תחילה), או `title` (אלפביתי).
  has_comments: true # בוליאני | אם true, החזר רק דפים שבהם לפחות תגובה אחת.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]

---