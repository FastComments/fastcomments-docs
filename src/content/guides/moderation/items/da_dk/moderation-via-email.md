---
FastComments understøtter et dagligt, ugentligt eller månedligt e‑mail‑oversigtsbrev for moderatorer og administratorer.

Hyppigheden kan konfigureres <a href="" target="_blank">her</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Rediger notifikationsformular, hvor oversigtsbrevet er indstillet til at ankomme dagligt, ugentligt eller månedligt'; title='Konfigurering af hyppighed for oversigtsbrev' app-screenshot-end]

Samtidig med at den inkluderer overordnede statistikker omkring dine kommentarer, vil den også vise de tre seneste kommentarer, der kræver gennemgang.

For hver af de nævnte kommentarer gives direkte magiske links til:
- Godkend kommentaren.
- Marker kommentaren som gennemgået og gå til svar‑siden.
- Marker kommentaren som spam.

Disse links for hver kommentar vil automatisk godkende dig og udføre handlingen fra din e‑mail.

Derudover er en **Moderer kommentarer**‑knap placeret i oversigtsbrevet, som udfører den samme godkendelse og fører dig til siden for moderering af kommentarer.

Bemærk venligst, at disse magiske links udløber efter et stykke tid.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Månedligt oversigts-e-mail med kommentarstatistikker og tre kommentarer, der kræver gennemgang, hver med godkend, svar og spam-links'; title='Oversigts-e-mail' app-screenshot-end]

#### Notifikationstyper

FastComments sender flere typer e‑mails til moderatorer og administratorer. Hvis ønsket, kan du fravælge `Comment Reply`‑notifikationer, mens du stadig modtager `New Comment`‑notifikationer ved at vælge de relevante indstillinger på siden `Edit Notifications`, som vist ovenfor.

---