---
Markierte Benutzer erhalten eine E‑Mail, die sie darüber informiert, dass sie in einem Kommentar markiert oder erwähnt wurden.

[app-screenshot-start url='/test-e2e/email/comment-user-mention?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"Hey%20%40winrid%20I%20wanted%20you%20to%20see%20this."%2C"commentHTML"%3A"Hey%20<b>%40winrid<%2Fb>%20I%20wanted%20you%20to%20see%20this."%2C"date"%3A1633998787864%2C"pageTitle"%3A"Some%20Page%20Title"%7D&username=winrid&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&INTRO=Hey%20winrid%2C&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&unsubscribeLink=%7B"url"%3A"%2Fauth%2Fmy-account%2Fedit-notifications"%2C"textId"%3A"UNSUBSCRIBE_HERE"%7D&viewCommentUrl=https%3A%2F%2Fexample.com%23fast-comments-jt%3Dsome-db-id&locale=en_us&canReplyByEmail=true&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Benachrichtigungs-E-Mail-Body, der einen Kommentar mit der @winrid-Erwähnung fett markiert zitiert, plus Ansicht- und Abmeldelinks'; title='Benachrichtigung über Benutzererwähnung' app-screenshot-end]

Das Deaktivieren von Benachrichtigungen verhindert diese E‑Mails, und in jeder E‑Mail wird ein Header bereitgestellt, damit E‑Mail‑Clients dem Benutzer ein nahtloses Abbestellen ermöglichen.

Markierte Benutzer erhalten außerdem eine In‑App‑Benachrichtigung, selbst wenn keine E‑Mail‑Adresse hinterlegt ist.

Wartet ein Kommentar auf die Genehmigung durch einen Moderator, wird die Erwähnungs‑E‑Mail zurückgehalten und erst nach Genehmigung des Kommentars gesendet. Sie wird nicht gesendet, wenn der Kommentar als Spam markiert, gelöscht oder nicht innerhalb von 7 Tagen genehmigt wird.

Um zu sehen, ob ein erwähnter Benutzer eine E‑Mail erhalten hat oder warum nicht, öffnen Sie die [Kommentarprotokolle](/guide-moderation.html#comment-logs) des Kommentars auf der Seite „Kommentare moderieren“.

---