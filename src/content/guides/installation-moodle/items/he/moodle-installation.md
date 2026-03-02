#### הורדת התוסף

הורידו את קובץ ה-ZIP של הגרסה האחרונה מ-<a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">מאגר ה-GitHub של FastComments עבור Moodle</a>.

#### חלץ לתיקיית ה-Moodle שלך

חלץ את קובץ ה-ZIP אל התקנת ה-Moodle שלך כך שהתוסף יהיה ב-`<moodle-root>/local/fastcomments`. תיקיית התוסף צריכה להכיל את `version.php`, `lib.php` וקבצי תוסף אחרים ישירות (לא מקוננים בתת-תיקייה).

לדוגמה:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### התקנה דרך מנהל האתר של Moodle

התחבר כמנהל אתר ונווט אל **ניהול אתר > התראות**. Moodle יזהה את התוסף החדש ויבקש ממך להפעיל את ההתקנה.

#### הגדרת התוסף

לאחר ההתקנה, עבור אל **ניהול אתר > תוספים > תוספים מקומיים > FastComments** כדי להזין את ההגדרות שלך. ראה את הסעיף [הגדרות](#items-moodle-configuration) לפרטים על כל אפשרות.