#### הורד את התוסף

הורד את קובץ ה-ZIP של הגרסה האחרונה מה<a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">מאגר FastComments Moodle ב-GitHub</a>.

#### חלץ לתיקיית ה-Moodle שלך

חלץ את קובץ ה-ZIP לתוך התקנת ה-Moodle שלך כך שהתוסף יהיה ב- `<moodle-root>/local/fastcomments`. תיקיית התוסף צריכה להכיל את `version.php`, `lib.php`, וקבצי התוסף האחרים ישירות (לא מקוננים בתיקיית משנה).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### התקן דרך מנהל האתר של Moodle

התחבר כמנהל אתר ונווט ל- **ניהול אתר > הודעות**. Moodle יזהה את התוסף החדש ויבקש ממך להריץ את ההתקנה.

#### הגדר את התוסף

לאחר ההתקנה, עבור ל- **ניהול אתר > תוספים > תוספים מקומיים > FastComments** כדי להזין את ההגדרות שלך. ראה את הסעיף [תצורה](#moodle-configuration) לפרטים על כל אפשרות.

---