FastComments unterstützt ein tägliches, wöchentliches oder monatliches E-Mail-Digest für Moderatoren und Administratoren.

Die Frequenz kann <a href="" target="_blank">hier</a> konfiguriert werden.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Edit Notifications-Formular, in dem das Digest auf tägliche, wöchentliche oder monatliche Zustellung eingestellt wird'; title='Konfiguration der Digest-Frequenz' app-screenshot-end]

Während es Gesamtstatistiken zu Ihren Kommentaren enthält, listet es außerdem die drei neuesten Kommentare, die einer Überprüfung bedürfen.

Für jeden dieser Kommentare werden direkte Magic-Links bereitgestellt, um:
- Den Kommentar zu genehmigen.
- Den Kommentar als überprüft zu markieren und zur Antwortseite zu gehen.
- Den Kommentar als Spam zu markieren.

Diese Links für jeden Kommentar authentifizieren Sie automatisch und führen die Aktion aus Ihrer E-Mail aus.

Zusätzlich befindet sich im Digest ein „Kommentare moderieren“-Button, der dieselbe Authentifizierung durchführt und Sie zur Seite „Kommentare moderieren“ führt.

Bitte beachten Sie, dass diese Magic-Links nach einiger Zeit ablaufen.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Monatliche Digest-E-Mail mit Kommentarstatistiken und drei zu überprüfenden Kommentaren, jeweils mit Genehmigen-, Antworten- und Spam-Links'; title='Digest-E-Mail' app-screenshot-end]

#### Benachrichtigungstypen

FastComments sendet mehrere Arten von E-Mails an Moderatoren und Administratoren. Wenn gewünscht, kann man `Comment Reply`-Benachrichtigungen abbestellen, während man weiterhin `New Comment`-Benachrichtigungen erhält, indem man die entsprechenden Optionen auf der oben gezeigten `Edit Notifications`-Seite wählt.

---