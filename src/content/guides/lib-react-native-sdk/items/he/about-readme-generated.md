ספרייה זו היא מימוש מלא של react-native עבור [FastComments](https://fastcomments.com).

היא תומכת בהערות בזמן אמת, צ'אט, שרשורים (threads), אימוג'ים, התראות, SSO, סקינים, והתאמה אישית מלאה על ידי העברת אובייקט גיליון סגנונות. כל הנכסים
ניתנים גם הם להתאמה אישית, והיא תומכת במעבר בין נכסים שונים בהתאם למצב כהה.

היתרון של ספרייה זו הוא שהיא גמישה יותר מאשר ה-wrapper `fastcomments-react-native`. ההערות מוצגות עם רכיבים נייטיב במקום בתוך webview.

הכול רץ על ה-backend של FastComments, כך שעליך לשלב רק את ה-UI:

```tsx
    <FastCommentsLiveCommenting config={config} styles={styles} callbacks={callbacks} assets={assets}/>
```

עיין ב-[example/src](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src) לדוגמאות נוספות.

הוסף צ'אט חי לאפליקציית React Native קיימת שלך, או אפילו בנה רשת חברתית!