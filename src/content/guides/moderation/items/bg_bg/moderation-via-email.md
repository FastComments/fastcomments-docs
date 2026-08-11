FastComments поддържа дневен, седмичен или месечен имейл дайджест за модератори и администратори.

Честотата му може да се конфигурира <a href="" target="_blank">тук</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Формуляр за редактиране на известия, където дайджестът е настроен да пристига дневно, седмично или месечно'; title='Конфигуриране на честотата на дайджеста' app-screenshot-end]

Докато включва общи статистики за вашите коментари, той също ще изброи трите най-скорошни коментара, нуждаещи се от преглед.

За всеки от тези коментари се предоставят директни магически връзки за:
- Одобряване на коментара.
- Маркиране на коментара като прегледан и отиване към страницата за отговор.
- Маркиране на коментара като спам.

Тези връзки за всеки коментар ще ви удостоверят автоматично и ще извършат действието от вашия имейл.

Освен това, бутон „Модериране на коментари“ се намира в дайджеста, който ще извърши същото удостоверяване и ще ви отведе до страницата за модериране на коментари.

Моля, имайте предвид, че тези магически връзки изтичат след определено време.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Месечен имейл дайджест със статистика за коментари и три коментара, нуждаещи се от преглед, всеки с връзки за одобряване, отговор и спам'; title='Имейл дайджест' app-screenshot-end]

#### Notification Types

FastComments изпраща различни типове имейли до модератори и администратори. При желание е възможно да се откажете от известия за `Comment Reply`, като все още получавате известия за `New Comment`, като изберете съответните опции в страницата `Edit Notifications`, показана по-горе.