Yorum widget'ının her örneği izole edilmiştir. Bu nedenle, FastComments doğası gereği sayfa başına birden fazla örneği veya aynı sohbet dizisini işaret eden birden fazla örneği destekler.

Örneğin VanillaJS kütüphanesi durumunda, yorum widget'ını farklı DOM düğümlerine bağlamanız yeterlidir. Sayfadaki geçerli diziyi güncellemek istiyorsanız, [Sayfayı Yeniden Yüklemeden Yorum Dizilerini Değiştirme](guide-customizations-and-configuration.html#switching-comment-threads) bölümüne bakın;

### Birden Fazla Örnek Arasında Kimlik Doğrulama Durumunu Senkronize Etme

Kendi yorum dizisine sahip sık sorulan sorular listesi olan özel bir tek sayfa uygulaması örneğini inceleyelim.

Bu durumda, DOM'da aynı anda birden fazla FastComments örneğimiz var.

Bu sorun değil, ancak kullanıcı deneyimi için bazı zorluklar ortaya çıkarır.

Bu akışı düşünün:

1. Kullanıcı, her biri kendi yorum widget'ına sahip sorular listesi olan bir sayfayı ziyaret eder.
2. Kullanıcı kullanıcı adını ve e-postasını girer ve dizilerden birine bir soru bırakır.
3. Sorusu olan başka bir SSS öğesi görürler.
4. Tekrar yorum yapmaya giderler. E-postalarını ve kullanıcı adlarını tekrar girmek zorundalar mı?

Bu durumda, FastComments widget örnekleri arasında kimlik doğrulama durumunu sizin için senkronize eder. Dördüncü adımda, aynı sayfada kullanıcı adını ve e-postasını girdikleri için kullanıcı zaten geçici olarak kimlik doğrulaması yapmış olacaktır.
