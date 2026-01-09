[related-parameter-start name = 'customCSS'; type = 'string'; related-parameter-end]

FastComments, özelleştirilmeye yönelik tasarlanmıştır. Yorum bileşeni güvenlik nedeniyle bir iframe içinde çalışır, bu yüzden özel stil uygulamak için iki yaklaşımdan birini takip etmelisiniz.

Birinci, en kolay ve bizim tercih ettiğimiz yaklaşım, [widget customization page](https://fastcomments.com/auth/my-account/customize-widget) kullanmaktır.

Widget özelleştirme sayfasında, "Show Advanced Options" bölümünü görün; bu bölümün altında "Custom CSS" adlı bir alan vardır:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.custom-css'; title='Custom CSS Input Area' app-screenshot-end]

Bu yaklaşımın bazı faydaları vardır:
1. Girilen CSS, kullanıcıya gönderilmeden önce küçültülür (minify edilir) ve düzenleme kullanıcı arayüzünde biçimlendirme tutarlı tutulur.
2. Widget özelleştirme kullanıcı arayüzünün tüm avantajlarından yararlanırsınız; örneğin farklı siteler için yorum bileşenini kolayca farklı şekilde özelleştirebilirsiniz.
3. Yorum bileşeninde değişiklik yaptığımızda, özel stiliniz sürüm yayın sürecimizin bir parçası olarak test edilecektir.

İkinci yaklaşım ise widget yapılandırmasında **customCSS** parametresini belirtmektir, şu şekilde:

[code-example-start config = {customCSS: "button { background: red; }" }; linesToHighlight = [6]; title = 'Passing Custom CSS'; code-example-end]

Ancak bunun *sınırlamaları* vardır:
1. Başlıkların (headers) boyutu nedeniyle sunucularımız isteği reddetmeden önce iletilebilecek özel CSS miktarı için bir sınır vardır.
2. Özel CSS'i kendi altyapınızda ve derleme sisteminizde yönetmelisiniz. Bu, bir dezavantajdan ziyade avantaj da olabilir.
3. Bu kullanım durumunda özel CSS'in ağ üzerinden **iki kez** gönderilmesi gerektiği için ek bir yük vardır; önce sunucularımıza gönderilir, sonra iframe içeriğinde geri gönderilir. Ancak çoğu yük boyutu için bu farkedilebilir değildir.
4. Yaygın bir optimizasyon, CSS'i ağ üzerinden gönderim boyutunu azaltmak için küçültmektir (minify); ancak bu yaklaşımda bunu sizin halletmeniz gerekir.
5. Özel CSS'iniz bizim değişiklik yaptığımızda test edilmeyecektir.

### External CSS Files

Dış bir dosyayı getirmesini widget'a `@import` kullanarak söyleyebilirsiniz!

`@import`'ı bir özelleştirme kuralına koymanız önerilir. Bu şekilde, yorum bileşeninde değişiklik yapmamız gerektiğinde, kurulumunuzu doğrulamak için otomasyon araçlarımızı kullanabiliriz. Örneğin, Widget Özelleştirme UI'da bir özelleştirme kuralı oluşturur, `Advanced`'a tıklar ve `Custom CSS` içine şunu girersiniz:

    @import url(https://example.com/styles.css);

#### In Code - Not Recommended

Harici bir CSS dosyasını ayrıca `customCSS` özelliği aracılığıyla da yükleyebilirsiniz:

[code-example-start config = {customCSS: "@import url(https://example.com/styles.css);" }; linesToHighlight = [6]; title = 'External CSS File'; code-example-end]

Ancak, bunu yaparsanız CSS'inizin bizim tarafımızdan test edilemeyeceğini unutmayın. 

### User Profile Modal Styling

Kullanıcı profil modal'ları da özel CSS ile stillendirilebilir. Ancak özel stilin kullanıcı profillerine uygulanmasını sağlamak için tüm CSS seçicilerinin `.user-profile` ile öneklenmiş olması gerekir. Bu önek olmadan, kullanıcı profil modal'ları için özel stil göz ardı edilecektir.

Örneğin:

[code-example-start config = {customCSS: ".user-profile .profile-name { color: blue; }" }; title = 'User Profile CSS'; code-example-end]

### Backwards Compatibility

FastComments'ta, müşterilerimizin yorum bileşenini özelleştirdiğini biliyoruz. Bu tasarımın bir parçasıdır - ürünümüzün ürününüzde tasarım tutarsızlıklarına neden olmasını istemeyiz.

Bu, ürünümüzün önemli bir parçası olduğundan, her sürümde müşteri başına yorum bileşenindeki değişiklikleri incelememizi sağlayan bir derleme hattımız (build pipeline) vardır.

Küçük sorunlar bulursak, sürümün sorunsuz gitmesini sağlamak için hesabınızı güncelleriz. Büyük kırılmaya yol açan değişiklikler görürsek, bu sürümü durdurmamıza olanak tanır.