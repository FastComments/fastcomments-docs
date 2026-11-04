Wenn ein Benutzer einen Kommentar oder eine Abstimmung hinterlässt und nicht angemeldet ist oder sein Konto nicht verifiziert ist, erhält er eine E-Mail, in der er gebeten wird, diese Aktion zu verifizieren.

Wir tun jedoch unser Bestes, um Ihre Benutzer nicht mit E-Mails zu spammen, und senden nicht mehr als eine Verifizierungs-E-Mail pro Sitzung. Weitere Details finden Sie im Abschnitt Sitzungen.

Standardmäßig sehen die Kommentar-Verifizierungs-E-Mails wie folgt aus:

[app-screenshot-start url='/test-e2e/email/commenter-verify-post?comment=%7B"commenterName"%3A"Alexander"%2C"comment"%3A"This%20is%20my%20comment."%2C"commentHTML"%3A"This%20is%20my%20comment."%2C"date"%3A1588812198540%7D&removedInDays=3&removedInDaysCount=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyPostUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Standard-Verifizierungs-E-Mail mit dem Kommentar von Alexander und einem Button zur Bestätigung des Beitrags'; title='Kommentar-Verifizierungs-E-Mail' app-screenshot-end]

Standardmäßig sehen die Abstimmungs-Verifizierungs-E-Mails wie folgt aus:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.content'; alt='Standard-E-Mail, die Devon bittet, eine Abstimmung zu bestätigen, mit dem abgestimmten Kommentar und einem Bestätigungs-Button'; title='Abstimmungs-Verifizierungs-E-Mail' app-screenshot-end]

Standardmäßig zeigt FastComments sein Logo und seinen Namen in der Fußzeile dieser E-Mails an:

[app-screenshot-start url='/test-e2e/email/commenter-verify-vote?vote=%7B"commenterName"%3A"Devon"%2C"comment"%3A"This%20is%20my%20comment.%20I%20totally%20left%20it%20intentionally.%20Yup."%2C"date"%3A1588812198540%7D&commenterName=Devon&url=some%20-%20url&removedInDays=3&FC_DOMAIN=https%3A%2F%2Ffastcomments.com&tenant=%7B"removeUnverifiedComments"%3Atrue%7D&verifyUrl=http%3A%2F%2Fexample.com&locale=en_us&API_KEY=T0ph%20123!'; linkUrl=false; selector = '.footer'; width = 700; alt='Unterer Teil einer Verifizierungs-E-Mail mit dem Standard-FastComments-Logo und -Namen in der Fußzeile'; title='E-Mail-Fußzeile' app-screenshot-end]

Wenn Sie den Tarif Flex oder Pro nutzen, können [Absendername, E-Mail und Branding angepasst werden](/guide-multiple-sites.html#from-name-email-logo).