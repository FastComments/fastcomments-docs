Yorumun net oy sayısı yapılandırılmış eşiğe ulaştığında tetiklenir. Net oy sayısı `votesUp - votesDown`'dır.

### Gerekli yapılandırma

- **Oy eşiği** - tamsayı >= 1. Tetkik, net oyları tam olarak bu sayıya getiren oyda çalışır.

Eşik 10 ise ve bir yorum net oy sayısını 9'dan 10'a çıkarırsa, tetik bir kez çalışır. Eğer bir oy daha gelip 10'dan 11'e çıkarırsa, tetik yeniden çalışmaz - eşikten sonraki her ek oyda tekrar tetiklenmez.

### Ajanın aldığı bağlam

- Güncel oy sayılarıyla birlikte yorum.
- Eşiğin aşılmasına neden olan oyun **yönü** (`up` veya `down`).
- Yapılandırmaya bağlı olarak isteğe bağlı konu / kullanıcı geçmişi / sayfa bağlamı.

### Dikkat edilmesi gerekenler

- Bir yorum 10'a çıkıp tekrar 9'a düşüp tekrar 10'a çıkarsa, tetik iki kez çalışır. Her yorum için "bir kez tetiklendi" durum kaydı yoktur - bu davranışa ihtiyaç duyuyorsanız, ajanın ilk çalıştırmada bir [hafıza notu](#tools-overview) kaydetmesini ve sonraki çalıştırmalarda bunu kontrol etmesini sağlayın.
- Eşik her zaman **net** oy sayısıdır, yalnızca olumlu oy sayısı değildir. 12 olumlu ve 2 olumsuz oyu olan bir yorum net 10'a sahiptir ve tetiklenir; 10 olumlu ve 0 olumsuz oyu olan bir yorum da tetiklenir.
- Sadece olumsuz oylarla eşik geçişleri mümkündür - bir yorum olumsuz oy nedeniyle 11'den 10'a düşerse bu da tetikler. Bağlamdaki `voteDirection` parametresi, eşik geçişinin hangi yönden olduğunu ajana bildirir.

### Yaygın kullanım alanları

- **Sabitleme** - [Top Comment Pinner template](#template-top-comment-pinner) bu tetik etrafında kuruludur.
- **Tanıtım / öne çıkarılan yorum iş akışları** - dış sistemlerin yorumu sitenizde başka bir yerde öne çıkarabilmesi için bir olayı [Webhooks](#webhooks-overview) aracılığıyla yayınlayın.
- **Etkileşim takibi** - yorum yazan kullanıcı hakkında bir hafıza notu kaydederek diğer ajanların popüler içerik ürettiklerini bilmesini sağlayın.

### İnce ayar

Doğru eşik topluluğa özgüdür. Birkaç gün boyunca düşük bir eşikte (5) [Çalıştırma Geçmişi](#run-history)'ni izleyerek ne sıklıkta tetiklendiğini görün. Tetiklenme sıklığı istediğiniz tempoya uyana kadar eşiği yükseltin.