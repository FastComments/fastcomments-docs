FastComments תומכת בתקציר דואר אלקטרוני יומי, שבועי או חודשי עבור מנחים (Moderators) ומנהלים (Administrators).

ניתן להגדיר את תדירות השליחה <a href="" target="_blank">כאן</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='טופס עריכת התראות שבו התקציר מוגדר להגיע יומית, שבועית או חודשית'; title='הגדרת תדירות התקציר' app-screenshot-end]

בנוסף לסטטיסטיקות כלליות על התגובות שלכם, התקציר יציג גם את שלוש התגובות האחרונות הממתינות לבדיקה.

עבור כל אחת מהתגובות הללו מסופקים קישורי קסם (magic links) ישירים ל:
- אישור התגובה.
- סימון התגובה כנבדקה ומעבר לדף התגובה.
- סימון התגובה כספאם.

קישורים אלו עבור כל תגובה יאמתו אוטומטית את זהותכם ויבצעו את הפעולה ישירות מהדואר האלקטרוני שלכם.

בנוסף, בתקציר מופיע כפתור "ניהול תגובות" (Moderate Comments) שמבצע את אותה אימות ומעביר אתכם
לדף ניהול התגובות.

שימו לב שקישורי הקסם הללו פגים לאחר זמן מה.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='דואר אלקטרוני של תקציר חודשי עם סטטיסטיקות תגובות ושלוש תגובות הממתינות לבדיקה, כל אחת עם קישורי אישור, תגובה וספאם'; title='דואר אלקטרוני של תקציר' app-screenshot-end]

#### סוגי התראות

FastComments שולחת סוגים שונים של דואר אלקטרוני למנחים ולמנהלים. במידת הרצון, ניתן לבטל את ההתראות מסוג `Comment Reply` ובמקביל
להמשיך לקבל התראות מסוג `New Comment` על ידי בחירת האפשרויות המתאימות בדף `Edit Notifications` המוצג למעלה.