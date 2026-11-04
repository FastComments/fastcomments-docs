כאשר משתמש משאיר תגובה או הצבעה, והוא אינו מחובר או שהחשבון שלו אינו מאומת, הוא יקבל אימייל המבקש ממנו לאמת פעולה זו.

עם זאת, אנו עושים כמיטב יכולתנו לא להציק למשתמשים שלך באימיילים, ולא נשלח יותר מאימייל אימות אחד לכל סשן. ראו את סעיף ה-Sessions לפרטים נוספים.

כברירת מחדל, אימיילי אימות התגובות נראים כך:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='גוף אימייל האימות המוגדר כברירת מחדל המצטט את התגובה של Alexander עם כפתור לאישור הפרסום'; title='אימייל אימות תגובה' app-screenshot-end]

כברירת מחדל, אימיילי אימות ההצבעות נראים כך:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='אימייל ברירת מחדל המבקש מ-Devon לאשר הצבעה, המציג את התגובה שהוצבעה עליה וכפתור אישור'; title='אימייל אימות הצבעה' app-screenshot-end]

כברירת מחדל, FastComments יציג את הלוגו והשם שלו בתחתית אימיילים אלה:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='תחתית אימייל אימות המציגה את הלוגו והשם של FastComments כפי שמוגדרים כברירת מחדל'; title='תחתית האימייל' app-screenshot-end]

אם אתם בתוכניות Flex או Pro, [את שם השולח, האימייל והמיתוג ניתן להתאים אישית](/guide-multiple-sites.html#from-name-email-logo).