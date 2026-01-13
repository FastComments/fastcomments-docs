כנראה שאינכם רוצים להגדיר את הconfig inline אם אתם מעבירים callbacks וכו'. במקום זאת, תרצו להגדיר
את הconfig בתוך בלוק `computed`, אחרת בכל פעם שה-callback שלכם ייקרא, ה-widget כולו יעבור רינדור מחדש.

[ראו את דוגמת ה-spinner כדי לראות כיצד לעשות זאת.](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-spinner.vue)