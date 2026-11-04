FastComments подржава дневни, недељни или месечни емаил преглед (digest) за Модераторе и Администраторе.

Учесталост се може подесити <a href="" target="_blank">овде</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Форма за уређивање обавештења где је преглед подешен да стиже дневно, недељно или месечно'; title='Подешавање учесталости прегледа' app-screenshot-end]

Поред укупне статистике о вашим коментарима, у прегледу ће бити наведена и три најновија коментара који захтевају преглед.

За сваки од наведених коментара постоје директни магични линкови за:
- Одобравање коментара.
- Означавање коментара као прегледаног и прелазак на страницу за одговор.
- Означавање коментара као спама.

Ови линкови за сваки коментар ће вас аутоматски аутентификовати и извршити акцију директно из вашег емаила.

Додатно, у прегледу се налази дугме Moderate Comments које врши исту аутентификацију и води вас на
страницу Moderate Comments.

Имајте у виду да ови магични линкови истичу након одређеног времена.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Месечни емаил преглед са статистиком коментара и три коментара који захтевају преглед, сваки са линковима за одобравање, одговор и спам'; title='Емаил преглед' app-screenshot-end]

#### Врсте обавештења

FastComments шаље више врста емаила Модераторима и Администраторима. Ако желите, могуће је искључити обавештења типа `Comment Reply`, а да и даље
примате обавештења типа `New Comment`, избором одговарајућих опција на страници `Edit Notifications` приказаној изнад.