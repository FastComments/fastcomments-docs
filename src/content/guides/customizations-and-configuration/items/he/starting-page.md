[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

בעת שליפת תגובות והצגתן, ווידג'ט התגובות צריך לדעת מאיזה עמוד להתחיל. ברירת המחדל היא שהוא מתחיל עם
העמוד הראשון, ומציג רק את אותו עמוד.

אם רצוי, ניתן להעביר לווידג'ט התגובות את מספר העמוד המדויק שיש להציג באמצעות ההגדרה *startingPage*.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

שים לב שמספרי העמודים מתחילים מ-0, ולכן הדוגמה שלמעלה מציגה את העמוד השני.

---