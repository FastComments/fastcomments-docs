Kullanıcılar yorum yaptığında veya oy verdiğinde ve oturum açmamış olduklarında, e-posta adreslerini ve kullanıcı adlarını girmeleri istenir.

Bazı siteler için, ziyaretçiden benzersiz bir kullanıcı adı uydurmasını istemek, özellikle mobilde bir engel oluşturabilir. FastComments, her yeni ziyaretçi için nötr bir kullanıcı adı oluşturabilir ve bunu kullanıcı adı alanına önceden doldurabilir; örneğin `BraveOtter4172` gibi.

Ziyaretçi bu adı olduğu gibi bırakabilir veya kendi seçtiği bir adla değiştirebilir.

Bu özellik, Customization UI üzerinden `Generate Usernames Automatically` adlı ayar altından etkinleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='Widget özelleştirme arayüzündeki Generate Usernames Automatically seçeneği'; title='Generate Usernames Automatically' app-screenshot-end]

#### Nasıl Davranır

- Oluşturulan her ad benzersizdir. Mevcut hesaplarla kontrol edilir ve o ziyaretçinin tarayıcı oturumu için ayrılır; böylece iki ziyaretçiye aynı ad önerilmez.
- Ad yalnızca henüz bir adı olmayan ziyaretçiler için oluşturulur. Oturum açmış kullanıcılar, SSO kullanıcıları ve daha önce yorum yapmış ziyaretçiler mevcut adlarını korur.
- [Anonim yorumlama](/guide-customizations-and-configuration.html#allow-anon) açıkken veya kapalıyken çalışır. Anonim yorumlama kapalıyken, ziyaretçi yine de e-postasını girer, ancak artık bir kullanıcı adı düşünmek zorunda kalmaz.
- Daha önce kullandığı bir e-postayı giren geri dönen ziyaretçi, mevcut hesabıyla eşleştirilir ve o hesaptaki adı korur.
- Ayrıca bir [Default Username](/guide-customizations-and-configuration.html#default-username) ayarlanmışsa, oluşturulan ad öncelik alır.