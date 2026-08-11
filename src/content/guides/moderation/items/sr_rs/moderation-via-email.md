FastComments подржава дневни, недељни или месечни имејл дигест за Модераторе и Администраторе.

Фреквенција се може подесити <a href="" target="_blank">овде</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Форма за уређивање обавештења где је дигест подешен да стиже дневно, неделно или месечно'; title='Подешавање фреквенције дигеста' app-screenshot-end]

Док укључује укупну статистику о вашим коментарима, такође ће приказати три најскорија коментара који захтевају преглед.

За сваки од тих коментара, директни магични линкови су доступни за:
- Одобравање коментара.
- Означавање коментара као прегледаног и отварање странице за одговор.
- Означавање коментара као спам.

Ови линкови за сваки коментар аутоматски ће вас аутентиковати и извршити радњу из вашег имејла.

Додатно, дугме „Moderate Comments“ се налази у Дигесту и извршиће исту аутентификацију и одвести вас на
страницу за модерирање коментара.

Имајте на уму да ови магични линкови истичу након неког времена.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Месечни дигест имејл са статистиком коментара и три коментара који захтевају преглед, сваки са линковима за одобравање, одговор и спам'; title='Дигест имејл' app-screenshot-end]

#### Notification Types

FastComments шаље више врста имејлова Модераторима и Администраторима. По жељи, могуће је одјавити се са обавештења о `Comment Reply`, док
се и даље примају обавештења о `New Comment` избором одговарајућих опција на страници `Edit Notifications` приказаној изнад.

---