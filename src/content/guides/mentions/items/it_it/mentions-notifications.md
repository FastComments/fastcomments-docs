---
Gli utenti taggati riceveranno un'email che li informa che sono stati taggati o menzionati in un commento.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Corpo dell\'email di notifica che cita un commento con la menzione @winrid in grassetto, più i link di visualizzazione e cancellazione dell\'iscrizione'; title='Notifica di menzione utente' app-screenshot-end]

Disattivare le notifiche impedirà l'invio di queste email, e un'intestazione è fornita in ogni email affinché i client di posta possano consentire all'utente di annullare l'iscrizione in modo fluido.

Gli utenti taggati ricevono anche una notifica in-app, anche se non hanno un indirizzo email registrato.

Se il commento è in attesa dell'approvazione del moderatore, l'email di menzione viene trattenuta e inviata una volta che il commento è approvato. Non viene inviata se il commento è contrassegnato come spam, eliminato o non approvato entro 7 giorni.

Per verificare se un utente menzionato ha ricevuto un'email, o perché non l'ha ricevuta, apri i [Log dei commenti](/guide-moderation.html#comment-logs) del commento dalla pagina Modera i commenti.

---