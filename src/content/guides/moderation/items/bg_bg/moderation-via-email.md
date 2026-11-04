FastComments поддържа дневен, седмичен или месечен имейл дайджест за модератори и администратори.

Честотата може да бъде конфигурирана <a href="" target="_blank">тук</a>.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Форма за редактиране на известията, където дайджестът е настроен да пристига ежедневно, седмично или месечно'; title='Конфигуриране на честотата на дайджеста' app-screenshot-end]

Освен общата статистика за вашите коментари, той ще изброи и трите най-скорошни коментара, изчакващи преглед.

За всеки от тези коментари са предоставени директни магически връзки за:
- Одобряване на коментара.
- Отбелязване на коментара като прегледан и преминаване към страницата за отговор.
- Отбелязване на коментара като спам.

Тези връзки за всеки коментар автоматично ще ви удостоверят и ще извършат действието директно от вашия имейл.

Освен това, в дайджеста има бутон „Moderate Comments", който извършва същото удостоверяване и ви отвежда до
страницата за модериране на коментари.

Моля, имайте предвид, че тези магически връзки изтичат след известно време.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Месечен имейл дайджест със статистика за коментарите и три коментара, изчакващи преглед, всеки с връзки за одобряване, отговор и спам'; title='Имейл дайджест' app-screenshot-end]

#### Типове известия

FastComments изпраща няколко вида имейли до модератори и администратори. Ако желаете, е възможно да се откажете от известията за `Comment Reply`, като същевременно
продължите да получавате известия за `New Comment`, като изберете съответните опции в страницата `Edit Notifications`, показана по-горе.