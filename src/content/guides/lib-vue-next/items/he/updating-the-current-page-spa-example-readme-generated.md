ב‑FastComments אנו קוראים למזהה המאמר, או לדף שאליו קשורות ההערות, URL ID כיוון שהוא יכול להיות url או ID.
הגדר את ה-URL ID באופן הבא. הקומפוננטה צופה בשינויים באובייקט config ותטען מחדש, כך שתוכל לעדכן את ה-URL ID.

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### אזור החשבון (שימו לב: לקוחות מהאיחוד האירופי)

אם החשבון שלך נמצא ב‑EU, הגדר `region = 'eu'` בתצורת הווידג'ט, לדוגמה:

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

אחרת, אין צורך להגדיר את `region`.