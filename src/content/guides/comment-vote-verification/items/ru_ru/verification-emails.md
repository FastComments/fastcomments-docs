---
Когда пользователь оставляет комментарий или голос, и при этом не вошёл в систему или его аккаунт не подтверждён,
он получит письмо с просьбой подтвердить это действие.

Тем не менее, мы делаем всё возможное, чтобы не спамить ваших пользователей письмами, и не будем отправлять более одного
письма с подтверждением за одну сессию. См. раздел «Сессии» для получения более подробной информации.

По умолчанию письма с подтверждением комментариев выглядят следующим образом:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Тело письма с подтверждением по умолчанию, цитирующее комментарий Александра с кнопкой подтверждения публикации'; title='Письмо подтверждения комментария' app-screenshot-end]

По умолчанию письма с подтверждением голосов выглядят следующим образом:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Письмо по умолчанию, запрашивающее у Девона подтверждение голоса, показывающее проголосованный комментарий и кнопку подтверждения'; title='Письмо подтверждения голоса' app-screenshot-end]

По умолчанию FastComments будет показывать свой логотип и название в подвале этих писем:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Нижняя часть письма с подтверждением, показывающая логотип FastComments по умолчанию и название в подвале'; title='Подвал письма' app-screenshot-end]

Если вы используете тарифы Flex или Pro, [Имя отправителя, электронная почта и брендинг могут быть настроены](/guide-multiple-sites.html#from-name-email-logo).