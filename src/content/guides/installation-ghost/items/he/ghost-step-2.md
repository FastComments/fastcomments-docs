כעת, לאחר שהורדנו את קובץ ה-zip, חלצו אותו לתיקייה. הורדתי את `casper.zip` ברירת המחדל וחילצתי אותו אל `Downloads\casper` ב-Windows.

לאחר מכן, ודאו שמותקנת אצלכם גרסת ה-LTS של NodeJS או חדשה יותר. ניתן להוריד אותה כאן: https://nodejs.org/en/download/

לאחר התקנת NodeJS, תרצו להתקין עורך קוד.

אנו ממליצים (וגם משתמשים) ב-Webstorm, שניתן להשיג כאן עם ניסיון חינם ל-30 יום (ללא צורך בכרטיס אשראי): https://www.jetbrains.com/webstorm/

האפשרות החינמית הטובה הבאה היא כנראה Visual Studio Code: https://code.visualstudio.com/download

לאחר שהגדרתם את העורך ופתחתם את תיקיית הערכת העיצוב בעורך, פתחו את הטרמינל ב-IDE והריצו:

[inline-code-attrs-start title = 'התקנת ערכת העיצוב'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

פלט מוצלח ייראה כך (ניתן להתעלם מאזהרות):

<div class="screenshot white-bg">
    <div class="title">פלט מוצלח של npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="פלט מוצלח של npm install" />
</div>

פעולה זו תגדיר את התלויות של ערכת העיצוב עבור הפקודות שנריץ בהמשך. בנוסף, הייצוא תלוי בכך שהתלויות של ערכת העיצוב מותקנות, אחרת הייבוא מחדש לא יעבוד כראוי.