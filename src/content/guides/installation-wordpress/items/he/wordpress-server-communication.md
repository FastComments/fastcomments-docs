כדי שהתוסף יעבוד, token נשמר בבסיס הנתונים של WordPress שלך וגם בחשבון FastComments שלך. כאשר התוסף מבצע בקשה לשרתים שלנו, הוא מספק
את ה-token הזה.

באפשרותך לצפות בכל האינטגרציות שמאושרות לחשבון FastComments שלך [כאן](https://fastcomments.com/auth/my-account/manage-data/integrations).

כל התקשורת מתבצעת דרך HTTPS.

כל התקשורת היא *יוצאת* משרת ה-WordPress שלך *אל* FastComments.com, כולל הסנכרון *חזרה* להתקנת ה-WordPress שלך כפי שמיושם
באמצעות [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) מתוך הגדרת [cron](https://developer.wordpress.org/plugins/cron/) בהתקנת ה-WordPress שלך.