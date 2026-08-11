FastComments podpira dnevni, tedenski ali mesečni e‑poštni povzetek za moderatorje in skrbnike.

Pogostost lahko nastavite <a href="" target="_blank">tukaj</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Obrazec Uredi obvestila, kjer je povzetek nastavljen, da prispeva dnevno, tedensko ali mesečno'; title='Nastavitev pogostosti povzetka' app-screenshot-end]

Poleg splošne statistike o vaših komentarjih bo povzetek tudi izpisal tri najnovejše komentarje, ki potrebujejo pregled.

Za vsak od teh komentarjev so na voljo čarobne povezave za:
- Odobritev komentarja.
- Označitev komentarja kot pregledanega in prehod na stran za odgovor.
- Označitev komentarja kot neželenega.

Te povezave za vsak komentar vas bodo samodejno avtenticirale in izvedle dejanje iz vašega e‑sporočila.

Poleg tega je v povzetku gumb **Moderiraj komentarje**, ki izvede isto avtentikacijo in vas odpelje na stran **Moderiraj komentarje**.

Upoštevajte, da te čarobne povezave po določenem času potečejo.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Mesečno e-poštno povzetek z statistiko komentarjev in tremi komentarji, ki potrebujejo pregled, vsak s povezavami za odobritev, odgovor in označitev kot neželeno'; title='E-poštni povzetek' app-screenshot-end]

#### Vrste obvestil

FastComments pošilja več vrst e‑poštnih sporočil moderatorjem in skrbnikom. Po želji se lahko odjavite od obvestil `Comment Reply`, medtem ko še vedno prejemate obvestila `New Comment`, tako da izberete ustrezne možnosti na strani `Edit Notifications`, prikazani zgoraj.