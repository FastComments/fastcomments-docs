FastComments yerelleştirilmiş bir platformdur. Tüm widget'larımız, e-postalarımız ve bildirimlerimiz yerelleştirilmiştir.

Yerelleştirilmiş olması, kullanıcının konumuna
ve tercih ettiği dile göre farklı bir dil ve biçimlendirme gösterdiğimiz anlamına gelir. Bunu kullanıcının tarayıcısının sağladığı bilgilere dayanarak belirleriz.

E-postadaki metni özelleştirmek için `Translations` sekmesine gidip bir `Locale`
seçebilir ve metni düzenleyebilirsiniz. Varsayılandan farklı olan metin kullanıcı arayüzünde vurgulanır. İsterseniz
yerel ayarlar arasında geçiş yapabilir ve sonunda kaydederek değişiklikleri kaybetmezsiniz.

Yerelleştirilmiş metne `TEXT` nesnesi üzerinden erişilir, örneğin: `<%= TEXT.INTRO %>`.

### SSO Notu

SSO entegrasyonlarında, `locale` belirtilmemişse, kullanıcı farklı bir `locale` ile yorum bileşenine
eriştiği her seferinde bu değer güncellenecektir. Bu, onların dil tercihinin
otomatik olarak güncellendiği ve gelecek e-postaların o `locale` ile gönderileceği anlamına gelir.

Bu ayrıca SSO payload'unda `locale` sağlayarak elle de ayarlanabilir.