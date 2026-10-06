---
Означени корисници ће добити е‑поруку која их обавештава да су означени, или поменути, у коментару.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Тело обавештења е‑поште које цитира коментар са @winrid помињем у подебљаном, плус линкови за преглед и одјаву'; title='Обавештење о помињању корисника' app-screenshot-end]

Искључивање обавештења ће спречити ове е‑поруке, а у свакој поруци је укључено заглавље које клијентима е‑поште омогућава да кориснику омогуће лако одјављивање.

Означени корисници такође добијају обавештење у апликацији, чак и ако немају регистровану е‑адресу.

Ако је коментар на чекању за одобрење модератора, е‑порука о помињању се задржава и шаље након што се коментар одобри. Не шаље се ако је коментар означен као спам, обрисан, или није одобрен у року од 7 дана.

Да бисте видели да ли је помињени корисник добио е‑поруку, или зашто није, отворите [Записнике коментара](/guide-moderation.html#comment-logs) за тај коментар са странице „Модерирање коментара“.

---