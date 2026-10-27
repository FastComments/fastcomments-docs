---
למרות שהשם שלו הוא כלי המיגרציה של תחום, ניתן גם להשתמש בו כדי להעביר תגובות בין דפים בודדים.

פשוט הזן את ערכי ה-[urlId](/guide-customizations-and-configuration.html#url-id) הישנים והחדשים בשדות הקלט `from` ו-`to`.

### העברת דפים באמצעות API

כדי להעביר מספר רב של דפים בבת אחת, השתמש ב-[`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post). הוא מקבל עד 100 זוגות של `from` ו-`to` לכל בקשה ומריץ אותם כמשימת אצווה.

כאשר `type` מוגדר ל-`urlId`, רק הדף שה-`urlId` שלו הוא בדיוק `from` מועבר, ולכן `post-1` אינו תואם ל-`post-10`. כלי הלוח מחוונים אינו כולל אפשרות זו.

כל מיגרציית API ניתן לבטל במשך 90 יום באמצעות [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post).

---