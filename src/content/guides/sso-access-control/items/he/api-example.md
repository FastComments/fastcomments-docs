כאן נעבור שלב‑אחר‑שלב על קריאה ל-API של FastComments כדי להגדיר בקרת גישה.

לפני שנתחיל, שימו לב שאיננו צריכים ליצור במפורש מבנה `Group`. קבוצות הן פשוט מזהים
נוספים ל-`Users` ו-`Pages`. הוספת קבוצה למשתמש או לדף יוצרת את הקבוצה באופן אוטומטי.

ראשית, ניצור שני משתמשים, `User A` ו-`User B`, — נתחיל אותם בתוך `Group X`:

[inline-code-attrs-start title = 'דוגמת cURL ליצירת משתמש A'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-a",
	"username": "User A",
	"email": "usera@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'דוגמת cURL ליצירת משתמש B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-b",
	"username": "User B",
	"email": "userb@example.com",
    "groupIds": ["GROUP-X"]
}'
[inline-code-end]

עכשיו ניצור `Page`. נקרא לה `Confidential Page`, ובינתיים אף אחד מהמשתמשים האלה לא יקבל גישה אליה כיוון שהיא תהיה בקבוצה `CONFIDENTIAL`:

[inline-code-attrs-start title = 'דוגמת cURL ל-POST של דף'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Confidential Page",
	"url": "https://mysite.com/confidential",
	"urlId": "https://mysite.com/confidential",
	"accessibleByGroupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

משתמשים A ו-B כרגע **אין להם** גישה לדף החדש. עם זאת, מכיוון שהם באותה קבוצה, `GROUP-X`, הם יכולים לבצע `@mention` זה לזה.

נעדכן את `User B` כדי שיוכל כעת לגשת לדף:

[inline-code-attrs-start title = 'דוגמת cURL לעדכון משתמש B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["GROUP-X", "CONFIDENTIAL"]
}'
[inline-code-end]

`User B` כעת שייך לשתי הקבוצות. המשתמשים שלנו עדיין יכולים לבצע `@mention` זה בזה, אך רק `User B` יכול לצפות בדף הסודי שלנו.

נעשה כך ש-`User B` יוכל לצפות רק בדף הסודי:

[inline-code-attrs-start title = 'דוגמת cURL לעדכון משתמש B'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "groupIds": ["CONFIDENTIAL"]
}'
[inline-code-end]

כעת הוא יכול לצפות בדף הסודי, אך אף אחד מהמשתמשים שלנו לא יכול לבצע `@mention` זה בזה, כיוון שהם בקבוצות שונות.

עם זאת, כל משתמש שאינו חלק מבקרת הגישה **יהיה מסוגל לגשת לדף שלנו**. כדי למנוע זאת, וודאו שאף משתמש SSO לא מוגדר עם `groupIds` כ-null. לדוגמה, ניצור את `User C`, שיש לו גישה להכל:

[inline-code-attrs-start title = 'דוגמת cURL ליצירת משתמש C'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "user-c",
	"username": "User C",
	"email": "userc@example.com",
    "groupIds": null
}'
[inline-code-end]

על ידי הגדרת `groupIds` כ-null, אנו מצביעים שהם לא מוגבלים על ידי בקרת הגישה.

עכשיו, ניצור דף שלכולם יש גישה אליו:

[inline-code-attrs-start title = 'דוגמת cURL ל-POST של דף'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Public Page",
	"url": "https://mysite.com/public",
	"urlId": "https://mysite.com/public",
	"accessibleByGroupIds": null
}'
[inline-code-end]

על ידי הגדרת `accessibleByGroupIds` כ-null, אנו מצהירים שה-`Page` הזה אינו נשלט על ידי בקרת גישה, ושני המשתמשים יכולים לגשת אליו.

זה מסיים את הסקירה שלנו של ה-API עבור בקרת גישה.