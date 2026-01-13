[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

כברירת מחדל, FastComments מאפשר למשתמשים להעלות תמונות. כאשר משתמש לוחץ על תמונה זו, FastComments, כברירת מחדל,
תפתח כרטיסייה חדשה כדי להציג את התמונה במלואה. הגדרת הדגל הזה ל-true מבטלת התנהגות זו:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

אם אינך מתכנן ללכוד את לחיצת התמונה בעצמך (ראה [onImageClicked](#callbacks)), אנו ממליצים לשלב זאת עם מעט עיצוב
כדי להסיר את המראה שהתמונה נראית ניתנת ללחיצה.