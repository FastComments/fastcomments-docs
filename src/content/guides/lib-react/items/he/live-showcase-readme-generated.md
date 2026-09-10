---
כדי לראות כל וידג'ט וזרימה פועלים מקומית נגד השוכר הציבורי `demo`, יש לשכפל את המאגר ולהריץ:

```bash
npm install
npm run build
cd examples/example-showcase
npm install
npm run dev
```

הדוגמאות מקשרות לספרייה דרך `file:../..`, ולכן שלב `npm run build` הראשי נדרש פעם אחת כדי לייצר את `dist/`.

לכל וידג'ט/זרימה יש את התצוגה שלו תחת `examples/example-showcase/src/views/` שניתן להעתיק ישירות לאפליקציית React שלך.
---