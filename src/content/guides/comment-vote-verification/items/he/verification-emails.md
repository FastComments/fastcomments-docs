---
כאשר משתמש משאיר תגובה או הצבעה, ואם הם לא מחוברים, או שהחשבון שלהם
לא מאומת, הם יקבלו דוא"ל המבקש לאמת את הפעולה הזו.

עם זאת, אנו עושים כמיטב יכולתנו לא להטריד את המשתמשים שלך בדוא"לים, ולא נשלח יותר מאשר
דוא"ל אימות אחד לכל סשן. עיין בסעיף הסשנים לפרטים נוספים.

כברירת מחדל, הודעות הדוא"ל לאימות תגובה נראות כך:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Comment Verification Email' app-screenshot-end]

כברירת מחדל, הודעות הדוא"ל לאימות הצבעה נראות כך:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; title='Vote Verification Email' app-screenshot-end]

כברירת מחדל, FastComments יציג את הלוגו ושמו בכותרת התחתונה של הודעות דוא"ל אלה:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; title='Email Footer' app-screenshot-end]

אם אתה על תוכניות Flex או Pro, [ניתן להתאים אישית את שם השולח, כתובת הדוא"ל והמיתוג](/guide-multiple-sites.html#from-name-email-logo).

---