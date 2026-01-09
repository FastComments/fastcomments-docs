---
FastComments 支援每日、每週或每月的郵件摘要，供版主與管理員使用。

其頻率可以在 <a href="" target="_blank">此處</a> 設定。

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; title='Configuring Digest Frequency' app-screenshot-end]

除了包含整體評論統計之外，它還會列出三則最近需要審核的評論。

對於每則該評論，會提供直接的魔法連結來：
- 批准該評論。
- 將該評論標記為已審核並前往回覆頁面。
- 將該評論標記為垃圾郵件。

每則評論的這些連結會自動替你進行驗證，並在郵件中執行該操作。

此外，摘要中有一個按鈕會執行相同的驗證，並將你帶到
審核評論頁面。

請注意，這些魔法連結會在一段時間後過期。

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; title='Digest Email' app-screenshot-end]

#### Notification Types

FastComments 向版主與管理員發送多種類型的電子郵件。若需要，您可以選擇不接收 `Comment Reply` 通知，同時仍透過在上方顯示的 `Edit Notifications` 頁面中選擇適當的選項來接收 `New Comment` 通知。

---