להלן קוד Vanilla JS להתקנת וידג'ט סיכום. ספריית React גם כוללת וידג'ט זה.

[inline-code-attrs-start title = 'התקנת וידג\'ט סיכום'; type = 'html'; isFunctional = true; type = 'html';  inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-reviews-summary.min.js"></script>
<div id="summary-widget"></div>
<script>
    window.FastCommentsReviewsSummaryWidget(document.getElementById('summary-widget'), {
        tenantId: 'demo'
    });
</script>
[inline-code-end]

הוידג'ט ימצא באופן אוטומטי את השאלות להצגה בסיכום בהתבסס על תצורת הווידג'ט המתאימה לעמוד/אתר זה.

### סינון תגובות לפי דירוג

כאשר וידג'ט התגובות של אותה העמוד נמצא גם הוא על העמוד, והוא מוגדר להציג את תשובת כל מגיב בתגובתו, שורות הפירוט הופכות ללחיצות. לחיצה על שורה, לדוגמה "5", מסננת את וידג'ט התגובות רק לתגובות שנתנו תשובה זו. לחיצה חוזרת על השורה, או על ה-X שבשבב "מציג רק" בוידג'ט התגובות, מציגה שוב את כל התגובות. זה אינו דורש תצורה: וידג'ט הסיכום מוצא את וידג'ט התגובות עם אותו `tenantId` ו-`urlId`.

זה פועל עבור שאלות כוכב, אגודל, אימוג'י, סולם ושאלות בחירה יחידה. הפירוט שהוידג'ט מציג מעל תיבת התגובה ניתן ללחיצה באותו אופן.

אם אתה זקוק לוידג'ט באחת הספריות האחרות שלנו שאין בה, פתח כרטיס תמיכה כדי שנדע להוסיף אותו.