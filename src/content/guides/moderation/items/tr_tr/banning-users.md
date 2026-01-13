Sitenizdeki kullanıcıların FastComments ile yorum yapmasını yasaklamanın iki yolu vardır.

İlki, eğer zaten e-posta adreslerini biliyorsanız, bunu <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">yasaklı kullanıcılar</a> sayfasına girebilirsiniz.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Bu sayfaya Yorumları Denetle -> Yasaklı Kullanıcılar üzerinden erişilebilir

Bir kullanıcıyı yasaklamaya gittiğimizde, Kalıcı veya Kalıcı Gölge Yasağı olmak üzere bir tür seçebiliriz:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

Bir kullanıcıyı yasaklamanın ikinci yolu, Yorum Moderasyon sayfasındaki her yorumun üzerine yerleştirilmiş yasaklama düğmesine tıklamaktır.

Yasak düğmesine tıkladığımızda, yasak türünü ve süresini belirleyebileceğimiz bazı seçenekler gösterilecektir.

### Gölge Yasakları

Gölge-yasak, kullanıcının yorumunun veya oylamasının başarılı bir şekilde kaydedildiğini gösteren ancak gerçekte kaydedilmediği izlenimini veren bir yasak türüdür. Bu, belirli durumlarda arzu edilebilir.

### IP Adresi ile Yasaklama

Bir tenant vazgeçmek istemediği sürece, FastComments, yorumcunun IP adresinin hashlenmiş bir versiyonunu saklayarak IP üzerinden yasaklamayı destekler.