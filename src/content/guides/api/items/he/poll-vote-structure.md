A `PollVote` הוא תשובת של אדם אחד לסקר. הספירות המוצגות על הסקר עצמו מתעדכנות בהתאם, ולכן אתה צריך אותם רק כאשר אתה רוצה לדעת *מי* הצביע על מה, במקום הסכומים הכוללים.

לבוחר יש לכל היותר הצבעה אחת לכל סקר. הצבעה חוזרת מעבירה את ההצבעה הקיימת שלו לאופציה החדשה במקום להוסיף הצבעה שנייה, ו-`updatedAt` מתעדת מתי זה קרה.

`voterId` הוא ה-`userId` כאשר הבוחר היה מחובר, ו-`anonUserId` אחרת.

[inline-code-attrs-start title = 'מבנה PollVote'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** The userId when the voter was logged in, otherwise the anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** When the voter last moved their vote to a different option. **/
    updatedAt?: string
}
[inline-code-end]

### פרטיות

הגדרת `privacy` של הסקר חלה על API זה באותו אופן שהיא חלה על וידג'ט ההערות:

- **אנונימי** (ברירת המחדל): אף אחד לא יכול לראות איך מישהו הצביע, ולכן ההצבעות אינן ניתנות לקריאה.  
  `GET /api/v1/poll-votes` ו-`GET /api/v1/poll-votes/:id` מחזירים `poll-anonymous`. הספירות של הסקר עדיין זמינות מ-`GET /api/v1/polls/:commentId`.
- **מנהלים ומפקחים**: מפתח ה-API שלך שייך למנהל האתר שלך, ולכן הוא יכול לקרוא את ההצבעות.
- **כולם**: ניתן לקרוא את ההצבעות.

ניתן לצמצם את פרטיות הסקר אך לא להרחיב אותה לאחר שיש הצבעות.