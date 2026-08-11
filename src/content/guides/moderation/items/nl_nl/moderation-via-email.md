FastComments ondersteunt een dagelijkse, wekelijkse of maandelijkse e‑maildigest voor moderators en beheerders.

De frequentie hiervan kan <a href="" target="_blank">hier</a> worden geconfigureerd.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Bewerk meldingen formulier waarin de digest is ingesteld om dagelijks, wekelijks of maandelijks te ontvangen'; title='Digestfrequentie configureren' app-screenshot-end]

Naast algemene statistieken over uw opmerkingen, wordt ook een lijst weergegeven van de drie meest recente opmerkingen die beoordeling nodig hebben.

Voor elk van deze opmerkingen worden directe magic‑links aangeboden om:
- De opmerking goedkeuren.
- De opmerking markeren als beoordeeld en naar de reageerpagina gaan.
- De opmerking markeren als spam.

Deze links voor elke opmerking zullen u automatisch authenticeren en de actie vanuit uw e‑mail uitvoeren.

Daarnaast bevindt zich in de digest een knop ‘Opmerkingen modereren’ die dezelfde authenticatie uitvoert en u naar de pagina ‘Opmerkingen modereren’ brengt.

Houd er rekening mee dat deze magic‑links na enige tijd verlopen.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Maandelijkse digest e‑mail met commentaarstatistieken en drie opmerkingen die beoordeling nodig hebben, elk met goedkeuren, reageren en spam links'; title='Digest e‑mail' app-screenshot-end]

#### Notificatietypen

FastComments stuurt verschillende soorten e‑mails naar moderators en beheerders. Indien gewenst kunt u zich afmelden voor `Comment Reply`‑meldingen, terwijl u nog steeds `New Comment`‑meldingen ontvangt door de juiste opties te kiezen op de pagina `Edit Notifications` die hierboven wordt getoond.

---