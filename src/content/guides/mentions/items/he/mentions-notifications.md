משתמשים שתויגו יקבלו אימייל המודיע להם שהם תויגו, או הוזכרו, בתגובה.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='גוף אימייל ההתראה המצטט תגובה עם האזכור של @winrid מודגש, וכן קישורי צפייה והסרה מרשימת התפוצה'; title='התראת אזכור משתמש' app-screenshot-end]

כיבוי ההתראות ימנע אימיילים אלו, ובכל אימייל מסופקת כותרת כך שלקוחות האימייל יוכלו לאפשר למשתמש להסיר את עצמו מרשימת התפוצה בצורה חלקה.

משתמשים שתויגו מקבלים גם התראה בתוך האפליקציה, גם כאשר אין להם כתובת אימייל רשומה.

אם התגובה ממתינה לאישור מנחה, אימייל האזכור נשמר ונשלח לאחר אישור התגובה. הוא לא נשלח אם התגובה סומנה כספאם, נמחקה, או לא אושרה בתוך 7 ימים.

כדי לראות אם משתמש שהוזכר קיבל אימייל, או מדוע לא קיבל, פתחו את [יומני התגובות](/guide-moderation.html#comment-logs) של התגובה מדף ניהול התגובות.