FastComments, Moderatörler ve Yöneticiler için Günlük, Haftalık veya Aylık e-posta özetini destekler.

Bu sıklık <a href="" target="_blank">burada</a> yapılandırılabilir.

[app-screenshot-start url='/auth/my-account/edit-notifications?demoDigestFrequencyValue=0'; linkUrl='/auth/my-account/edit-notifications'; selector = '.content form'; alt='Düzenleme Bildirimleri formu, özetin günlük, haftalık veya aylık olarak gelmesi ayarlanmıştır'; title='Özet Sıklığını Yapılandırma' app-screenshot-end]

Yorumlarınızla ilgili genel istatistikleri içerirken, aynı zamanda gözden geçirilmesi gereken en son üç yorumu da listeler.

Bu yorumların her biri için doğrudan sihirli bağlantılar sağlanır:
- Yorumu onayla.
- Yorumu incelendi olarak işaretle ve yanıt sayfasına git.
- Yorumu spam olarak işaretle.

Bu bağlantılar, her yorum için otomatik olarak kimliğinizi doğrular ve e-postanızdan eylemi gerçekleştirir.

Ayrıca, Özet içinde bir 'Yorumları Modere Et' düğmesi bulunur; bu aynı kimlik doğrulamayı yapar ve sizi Yorumları Modere Et sayfasına yönlendirir.

Lütfen bu sihirli bağlantıların bir süre sonra süresinin dolduğunu unutmayın.

[app-screenshot-start url='/test-e2e/email/tenant-comment-digest?HOST=http%3A%2F%2Flocalhost%3A3001&stats=%7B"hasHistory"%3Atrue%2C"newCommentsCount"%3A10002%2C"hasNewCommentsIncreased"%3Atrue%2C"hasNewCommentsDecreased"%3Afalse%2C"approvedCommentsCount"%3A44%2C"hasApprovedCommentsIncreased"%3Afalse%2C"hasApprovedCommentsDecreased"%3Atrue%2C"spamCommentsCount"%3A21%2C"hasSpamCommentsIncreased"%3Afalse%2C"hasSpamCommentsDecreased"%3Atrue%2C"newUsersCount"%3A30%2C"hasNewUsersIncreased"%3Atrue%2C"hasNewUsersFalse"%3Afalse%7D&BANNER_TEXT=FastComments%20Monthly%20Digest&commentCount=100000&hasCommentsNeedsReview=true&comments=%5B%7B"commenterName"%3A"Devon%20Winrick"%2C"commentHTML"%3A"This%20is%20a%20very%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Devon"%2C"commentHTML"%3A"This%20is%20a%20somewhat%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o.jpg"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%2C%7B"commenterName"%3A"Bob"%2C"commentHTML"%3A"This%20is%20a%20kind%20of%20recent%20comment%20that%20needs%20approval."%2C"date"%3A1588812198540%2C"locale"%3A"en_us"%2C"avatarSrc"%3A"https%3A%2F%2Ffastcomments.com%2Fimages%2Funknown-person.png"%2C"url"%3A"https%3A%2F%2Fstatic.fastcomments.com%2F1582299581264-69384190_3015192525174365_476457575596949504_o"%7D%5D&locale=en_us&digestEmail=%7B"tenantId"%3A"tenant-id"%2C"userId"%3A"user-id"%2C"_id"%3A"some-id"%2C"temporaryId"%3A"temporary-id"%7D&API_KEY=T0ph%20123!&rawTemporaryId=xyz'; linkUrl=false; selector = '.content'; alt='Yorum istatistikleri ve gözden geçirilmesi gereken üç yorum içeren aylık özet e-posta, her biri onay, yanıt ve spam bağlantılarına sahiptir'; title='Özet E-posta' app-screenshot-end]

#### Bildirim Türleri

FastComments, Moderatörler ve Yöneticilere birden fazla e-posta türü gönderir. İstenirse, `Comment Reply` bildirimlerinden çıkmak mümkündür, aynı zamanda yukarıda gösterilen `Edit Notifications` sayfasındaki uygun seçenekleri seçerek `New Comment` bildirimlerini almaya devam edebilirsiniz.

---