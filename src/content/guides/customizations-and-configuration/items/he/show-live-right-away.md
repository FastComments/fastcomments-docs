[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

כברירת מחדל, תגובות בזמן אמת מופעלות. משמעות הדבר היא שאם תגובות מוספות, נמחקות, נערכות או מוצמדות, השינויים צריכים להופיע
לכל המשתמשים הצופים בשרשור התגובות באותו זמן.

אולם, כברירת מחדל התגובות החדשות האלה יופיעו מתחת לכפתור שמוצג באופן דינמי עם טקסט דומה ל-"הצג 2 תגובות חדשות".

אם התגובות החדשות הן תגובות ישירות לעמוד, הכפתור יופיע בחלק העליון של שרשור התגובות. אם הן תגובות לתגובה מסוימת, 
הכפתור יופיע מתחת לאותה תגובה.

זאת כדי למנוע שינוי מתמיד בגודל הדף עבור המשתמש, מה שעלול לגרום לתסכול כאשר מנסים לתפוס את סרגל הגלילה.

במקרים מסוימים, כמו מכרזים חיים או אירועים מקוונים, זה אינו ההתנהגות הרצויה - ייתכן שתרצו שהווידג'ט של התגובות יהיה
יותר כמו תיבת "צ'אט" שבה תגובות חדשות "מוצגות מיד".

ולכן, שם הדגל שמפעיל תכונה זו: **showLiveRightAway**.

ניתן להפעיל זאת כך:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

ניתן להתאים זאת ללא קוד, בדף ההתאמה האישית של הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---