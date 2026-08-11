[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

בברירת מחדל, תגובות חיות מופעלות. משמעות הדבר היא שאם מתווספות, נמחקות, נערכות או מוצמדות תגובות, השינויים צריכים להופיע
לכל המשתמשים הצופים בחוט התגובות באותו זמן.

עם זאת, בברירת מחדל התגובות החדשות יופיעו תחת כפתור שמוצג דינמית עם טקסט דומה ל-"Show 2 New Comments".

אם התגובות החדשות הן תגובות ישירות לדף, הכפתור יופיע בראש חוט התגובות. אם הן תגובות לתגובה ספציפית,
הכפתור יופיע מתחת לאותה תגובה.

זה נעשה כדי למנוע שינוי מתמשך בגודל העמוד עבור המשתמש, שעלול לגרום לתסכול כשמנסים לתפוס את סרגל הגלילה.

בחלק מהמקרים, כמו מכרזים חיים או אירועים מקוונים, זה אינו ההתנהגות הרצויה - ייתכן שתרצו שהווידג'ט של התגובות יהיה
דומה יותר לתיבת "chat" שבה תגובות חדשות "show right away".

לכן, שם הדגל שמאפשר תכונה זו: **showLiveRightAway**.

ניתן להפעיל אותו כך:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

ניתן להתאים זאת ללא קוד, בעמוד התאמה אישית של הווידג'ט:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='הגדרת קיפול תגובות חיות הוחלפה כך שהתגובות החדשות מופיעות מיידית במקום מאחורי כפתור'; title='הצג תגובות חיות מייד' app-screenshot-end]