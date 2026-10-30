ניתן לנהל סקרים גם מקוד שלכם באמצעות ה‑REST API, לדוגמה כדי להוסיף סקר לתגובה מה‑backend שלכם, לסגור סקר, או לייצא את ההצבעות:

- [Poll Structure](/guide-api.html#poll-structure) מתאר את אובייקט הסקר ואת המגבלות שלו.  
- [GET](/guide-api.html#poll-get), [PUT](/guide-api.html#poll-put), [PATCH](/guide-api.html#poll-patch), ו-[DELETE](/guide-api.html#poll-delete) `/api/v1/polls/:commentId` קוראים, מחליפים, עורכים ומסירים את הסקר בתגובה.  
- [PollVote Structure](/guide-api.html#poll-vote-structure) מתאר הצבעה בודדת.  
- [GET](/guide-api.html#poll-votes-get), [POST](/guide-api.html#poll-vote-post), ו-[DELETE](/guide-api.html#poll-vote-delete) `/api/v1/poll-votes` מציגים, מבצעים ומסירים הצבעות.

ניתן גם ליצור סקר יחד עם התגובה שלו על‑ידי העברת `poll` ל‑`POST /api/v1/comments`.

ה‑API פועל עם אותם ההגדרות כמו הווידג׳ט: יש להפעיל סקרים, והצבעות ללא משתמש מחובר נדחות אלא אם מותרות הצבעות סקר אנונימיות. הצבעות בודדות בסקרים אנונימיים אינן ניתנות לקריאה דרך ה‑API.