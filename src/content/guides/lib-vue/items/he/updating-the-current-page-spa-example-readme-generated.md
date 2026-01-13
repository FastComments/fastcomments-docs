In FastComments אנחנו מכנים את מזהה הכתבה, או את העמוד שאליו קשורות התגובות, ה-URL ID מאחר שהוא יכול להיות url או ID.
הגדר את ה-URL ID בדרך הבאה. הקומפוננטה מאזינה לשינויים באובייקט config ותטען מחדש, כך שניתן פשוט לעדכן את ההגדרות "url" ו-"urlId".

ראה דוגמה עובדת מלאה [כאן](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts).

הפעל את דוגמת הפגינציה באמצעות:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### אזור החשבון (תשומת לב: לקוחות מהאיחוד האירופי)

אם החשבון שלך ממוקם באיחוד האירופי, הגדר את `region = 'eu'` בקונפיגורציית הווידג'ט, לדוגמה:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

אחרת, אין צורך להגדיר את `region`.