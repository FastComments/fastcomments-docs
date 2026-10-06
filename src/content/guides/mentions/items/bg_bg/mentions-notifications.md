---
Отбелязаните потребители ще получат имейл, който ги уведомява, че са били отбелязани или споменати в коментар.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Тяло на известие по имейл, цитиращо коментар с @winrid споменат в удебелен текст, плюс връзки за преглед и отписване'; title='Известие за споменат потребител' app-screenshot-end]

Изключването на известията ще предотврати тези имейли и във всеки имейл се предоставя заглавка, която позволява на клиентските програми за имейл потребителят да се отписва безпроблемно.

Отбелязаните потребители също получават известие в приложението, дори ако нямат записан имейл адрес.

Ако коментарът изчаква одобрение от модератор, имейлът за споменаване се задържа и се изпраща след одобрението на коментара. Той не се изпраща, ако коментарът е маркиран като спам, изтрит или не е одобрен в рамките на 7 дни.

За да видите дали споменатият потребител е получил имейл или защо не е получил, отворете [Журнали на коментарите](/guide-moderation.html#comment-logs) от страницата „Модериране на коментари“.

---