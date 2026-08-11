[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments özelleştirilebilecek şekilde tasarlanmıştır. Yorum widget'ı güvenlik nedenleriyle bir iframe içinde çalışır, bu yüzden özel stil uygulamak için iki yaklaşımdan birini izlemelisiniz.

İlk ve en kolay yaklaşım, bizim tercih ettiğimiz, [widget özelleştirme sayfasını](https://fastcomments.com/auth/my-account/customize-widget) kullanmaktır.

Widget özelleştirme sayfasında, "Show Advanced Options" (Gelişmiş Seçenekleri Göster) bölümüne bakın; altında "Custom CSS" (Özel CSS) olarak etiketlenmiş bir alan bulunur:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; alt='Widget özelleştirme sayfasında Gelişmiş Seçenekleri Göster altında Özel CSS editörü'; title='Özel CSS Giriş Alanı' app-screenshot-end]

Bu yaklaşımın bazı avantajları vardır:
1. Girilen CSS, kullanıcıya gönderilmeden önce küçültülür ve biçimlendirme düzenleme UI'sinde tutarlı kalır.
2. Widget özelleştirme UI'sinin tüm avantajlarını elde edersiniz; örneğin yorum widget'ını farklı siteler için kolayca özelleştirebilirsiniz.
3. Yorum widget'ında değişiklik yaptığımızda, özel stiliniz sürüm sürecimizin bir parçası olarak test edilir.

İkinci yaklaşım, widget yapılandırmasında **customCSS** parametresini aşağıdaki gibi belirtmektir:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Özel CSS Geçirme'; code-example-end]

Ancak, bunun *sınırlamaları* vardır:
1. Başlıkların boyutu nedeniyle, sunucularımız isteği reddedene kadar geçirilebilecek özel CSS miktarı için bir sınırlama vardır.
2. Özel CSS'i altyapınızda ve derleme sisteminizde yönetmelisiniz. Bu, bir dezavantajdan ziyade bir avantaj da olabilir.
3. Bu kullanım durumunda özel CSS'in ağ üzerinden **iki** kez gönderilmesi ek bir yük oluşturur; önce sunucularımıza, ardından iframe içeriğine gönderilir. Ancak çoğu yük boyutu için bu fark edilmez.
4. Yaygın bir optimizasyon, CSS'i küçülterek ağ üzerindeki boyutunu azaltmaktır; ancak bu yaklaşımda bunu kendiniz yönetmelisiniz.
5. Özel CSS'iniz, değişiklik yaptığımızda test edilmez.

### Harici CSS Dosyaları

`@import` kullanarak widget'ın harici bir dosya almasını sağlayabilirsiniz!

`@import`'u bir özelleştirme kuralına koymanız önerilir. Böylece, yorum widget'ında bir değişiklik yapmamız gerektiğinde otomasyon araçlarımızı kullanarak ayarlarınızı doğrulayabiliriz. Örneğin, Widget Özelleştirme UI'sinde bir özelleştirme kuralı oluşturur, `Advanced` (Gelişmiş) sekmesine tıklarsınız ve `Custom CSS` (Özel CSS) alanına girersiniz:

    @import url(https://example.com/styles.css);

#### Kod İçinde - Önerilmez

`customCSS` özelliği aracılığıyla da harici bir CSS dosyası yükleyebilirsiniz:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'Harici CSS Dosyası'; code-example-end]

Ancak, bunu yaparsanız CSS'inizin bizim tarafımızdan test edilemeyeceğini unutmayın.

### Kullanıcı Profili Modal Stili

Kullanıcı profili modalları da özel CSS ile stil verilebilir. Ancak, özel stilin kullanıcı profillerine uygulanmasını sağlamak için tüm CSS seçicileri `.user-profile` ile ön eklenmelidir. Bu ön ek olmadan, kullanıcı profili modalları için özel stil yoksayılır.

For example:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'Kullanıcı Profili CSS'; code-example-end]

### Geriye Dönük Uyumluluk

FastComments'ta, müşterilerimizin yorum widget'ını özelleştirdiğini biliyoruz. Bu tasarım gereği; ürünümüzün sizin ürününüzde tasarım tutarsızlıklarına yol açmasını istemeyiz.

Bu, ürünümüzün önemli bir parçası olduğundan, her sürümde müşteri bazında yorum widget'ındaki değişiklikleri incelememizi sağlayan bir derleme hattına sahibiz.

Küçük sorunlar bulursak, sürümün sorunsuz ilerlemesini sağlamak için hesabınızı güncelleriz. Büyük kırıcı değişiklikler görürsek, sürümü durdurmamıza olanak tanır.

---