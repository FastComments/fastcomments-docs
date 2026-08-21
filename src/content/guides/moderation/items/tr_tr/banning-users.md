FastComments ile sitenizdeki kullanıcıların yorum yapmasını yasaklamanın iki yolu vardır.

İlk yol, eğer e-posta adreslerini zaten biliyorsanız, bunu <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">yasaklı kullanıcılar</a> sayfasına girebilirsiniz.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Moderate Comments altında yasaklı kullanıcılar listesi, yasaklı e-posta adresleri ve yeni bir yasak eklemek için bir düğme'; title='Yasaklı Kullanıcılar Sayfası' app-screenshot-end]

Bu sayfaya Moderatör Yorumları -> Yasaklı Kullanıcılar üzerinden erişilebilir.

Bir kullanıcıyı yasaklamak istediğimizde, kalıcı veya kalıcı gölge yasak olmak üzere bir tür seçebiliriz:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Kalıcı veya Kalıcı Gölge Yasak seçeneğiyle bir e-posta alanı içeren yeni yasak formu'; title='Bir Kullanıcıyı Yasaklama' app-screenshot-end]

Kullanıcıyı yasaklamanın ikinci yolu, Yorum Moderasyonu sayfasındaki her yorumun üzerine yerleştirilmiş yasaklama düğmesine tıklamaktır.

Yasaklama düğmesine tıkladığınızda, yasak türünü ve süresini belirleyebileceğiniz bazı seçenekler karşınıza çıkar.

### E-posta Takma Adları

Bir kullanıcıyı e-posta ile yasaklarken, FastComments otomatik olarak `+` takma adlarını görmezden gelir. Örneğin, `user+alias@gmail.com` adresini yasaklamak, aynı zamanda `user@gmail.com` ve bu adresin diğer `+` varyasyonlarını, örneğin `user+other@gmail.com` adresini de yasaklar.

### Gölge Yasaklar

Gölge yasak, kullanıcının yorumunun veya oyununun başarılı bir şekilde kaydedildiği izlenimini veren, ancak gerçekte kaydedilmediği bir yasak türüdür. Bu, belirli durumlarda istenebilir.

### IP Adresi Üzerinden Yasaklama

Bir kiracı çıkmayı tercih etmedikçe, FastComments yorumcunun IP adresinin hashlenmiş bir sürümünü saklayarak IP üzerinden yasaklamayı destekler.

### Yasaklı Kullanıcıları Arama

Listeniz bir iki sayfayı aştığında, tablo üzerindeki arama satırıyla daraltabilirsiniz.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Yasaklı Kullanıcılar sayfasında bir Arama satırı, Search By açılır menüsü, Match açılır menüsü ve Value girişi'; title='Yasaklı Kullanıcıları Arama' app-screenshot-end]

Üç kontrol vardır:

- **Search By** hangi alanda arama yapılacağını seçer: Any Field, Email, Name, Banned By veya Banned For Saying. Son dört, tabloda aynı isimdeki sütunlara karşılık gelir.
- **Match** karşılaştırma şeklini seçer. **Contains** değerinizi alanda herhangi bir yerde bulur, **Equals** ise tüm alanla eşleşir.
- **Value** aranan metindir.

Tüm alanlar büyük/küçük harfe duyarsız eşleşir, bu yüzden `SPAMMER@EXAMPLE.COM` araması, `spammer@example.com` olarak saklanan bir yasağı bulur.

Bilmeniz gereken birkaç nokta:

- **Banned For Saying** kullanıcının yasaklanmasına neden olan yorum metnini arar. Bu, belirli bir ifadeyi içeren tüm yasaklıları bulmanızı sağlar.
- **Banned By** yasağı veren moderatörün adını arar, bu da başka bir moderatörün kararlarını gözden geçirmek için faydalıdır.
- Joker karakterli yasaklar `*` ile saklanır, bu yüzden `bademail.com` için bir **Contains** araması `*@bademail.com` yasaklamasını bulur.
- **Name** Name sütununda gösterilen isimle eşleşir, bu sayede yasaklandıktan sonra adını değiştirmiş bir kullanıcıyı ve yasaklamayı sadece e-posta girerek oluşturduğunuzda kaydedilmemiş bir ismi bile bulur. Yasakta kaydedilen isim de eşleşir, bu yüzden eski ya da mevcut isimle arama yapılabilir.
- **Any Field** e-posta, isim, yasaklayan moderatör ve yasaklanan yorum metnini birlikte arar.

Aramanız sayfa URL'sinin bir parçasıdır, bu yüzden filtrelenmiş bir listeyi diğer moderatörlerle diğer moderasyon bağlantılarını paylaştığınız gibi paylaşabilirsiniz. Sonuçlarda sayfalama yaparken arama uygulanmış kalır, yeni bir arama başlatmak sizi ilk sayfaya döndürür ve **Clear** tam listeye geri döner.