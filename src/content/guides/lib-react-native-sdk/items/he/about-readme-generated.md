הספרייה הזו היא מימוש מלא ל-react-native של [FastComments](https://fastcomments.com).

היא תומכת בתגובות בזמן אמת, בצ'אט, בשרשורים, באמוטיקונים, בהתראות, ב-SSO, ב-skins ובהתאמה מלאה על ידי העברת אובייקט stylesheet. כל ה-assets ניתנים גם הם להתאמה, והיא תומכת בהחלפת assets שונים בהתאם למצב כהה.

היתרון של ספרייה זו הוא שהיא גמישה יותר מה-wrapper `fastcomments-react-native`. התגובות מוצגות עם native components במקום בתוך webview. הערה: `react-native-webview` עדיין נדרשת כתלות טרנזיטיבית של עורך הטקסט המתקדם (`@10play/tentap-editor`).

הכל רץ על ה-backend של FastComments, כך שכל מה שעליכם לשלב הוא את ה-UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

ראה [example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) לעוד דוגמאות.

הוסיפו צ'אט בזמן אמת לאפליקציית React Native קיימת שלכם, או אפילו בנו רשת חברתית!