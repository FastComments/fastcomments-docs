FastComments podržava dnevni, tjedni ili mjesečni e‑mail sažetak za moderatore i administratore.

Učestalost se može konfigurirati <a href="" target="_blank">ovdje</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Obrazac Uredi obavijesti gdje je sažetak postavljen da stiže dnevno, tjedno ili mjesečno'; title='Konfiguriranje učestalosti sažetka' app-screenshot-end]

Uz prikaz ukupne statistike vaših komentara, također će navesti tri najnovija komentara koja zahtijevaju pregled.

Za svaki od tih komentara, pruženi su izravni magični linkovi za:
- Odobri komentar.
- Označi komentar kao pregledan i idi na stranicu za odgovor.
- Označi komentar kao spam.

Ti linkovi za svaki komentar automatski će vas autentificirati i izvršiti radnju iz vašeg e‑maila.

Dodatno, gumb Moderiraj komentare nalazi se u sažetku i izvršit će istu autentifikaciju te vas odvesti na stranicu Moderiraj komentare.

Imajte na umu da ovi magični linkovi isteknu nakon nekog vremena.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Mjesečni e‑mail sažetak s komentar statistikom i tri komentara koja zahtijevaju pregled, svaki s linkovima za odobrenje, odgovor i spam'; title='E‑mail sažetak' app-screenshot-end]

#### Vrste obavijesti

FastComments šalje više vrsta e‑mailova moderatorima i administratorima. Po želji, moguće je odjaviti se od obavijesti o `Odgovoru na komentar`, dok i dalje primate obavijesti o `Novom komentaru` odabirom odgovarajućih opcija na stranici `Uredi obavijesti` prikazanoj iznad.

---