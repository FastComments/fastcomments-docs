### Karanlık Mod Desteği

### Dinamik Karanlık Mod

Sitenizin karanlık modu body öğesine `.dark` sınıfı eklenerek kontrol ediliyorsa, Collab Chat kullanıcı arayüzü bunu yeniden başlatma gerektirmeden otomatik olarak dikkate alır. Widget'ın stilleri bu sınıfın varlığına tepki verecek şekilde tasarlanmıştır.

[inline-code-attrs-start title = 'Karanlık Modu CSS Örneği'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Karanlık mod CSS'iniz */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### CSS ile Özel Stil

Vurgulamalar, sohbet pencereleri ve diğer öğelerin görünümünü CSS kullanarak özelleştirebilirsiniz. Widget, stil sayfanızda hedefleyebileceğiniz belirli sınıflar ekler.

Metin vurgulamaları FastComments yorum balonu stil sistemi kullanır, bu nedenle standart yorum widget'ına uyguladığınız herhangi bir özelleştirme Collab Chat'i de etkileyecektir.

### Üst Çubuk Özelleştirme

Üst çubuk çevrimiçi kullanıcı sayısını ve tartışma sayısını gösterir. Konumunu `topBarTarget` olarak özel bir öğe sağlayarak özelleştirebilirsiniz:

[inline-code-attrs-start title = 'Özel Üst Çubuk Konumu'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Veya tamamen devre dışı bırakmak için `null` olarak ayarlayabilirsiniz:

[inline-code-attrs-start title = 'Üst Çubuğu Devre Dışı Bırak'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Mobil Davranışı

768px'den daha dar ekranlarda Collab Chat otomatik olarak mobil için optimize edilmiş bir düzene geçer. Sohbet pencereleri metnin yanında kayan yerine tam ekran görünür ve daha anında etkileşim için seçim gecikmesi kaldırılır.

Bu davranış dahili olarak mevcuttur ve herhangi bir yapılandırma gerektirmez. Widget ekran boyutunu otomatik olarak algılar ve buna göre ayarlar.

### Sohbet Penceresi Görünümü

Sohbet pencereleri masaüstünde 410px genişliğindedir ve vurgulanan metne işaret eden 16px'lik bir ok içerir. Pencereler, `to-right`, `to-left`, `to-top` ve `to-bottom` gibi konumlandırma sınıflarını kullanarak kullanılabilir görüntüleme alanı alanına göre otomatik olarak konumlanır.

Bu pencerelerin renklerini, yazı tiplerini, boşluklarını veya diğer görsel özelliklerini ayarlamak için özel CSS ekleyebilirsiniz. Sohbet pencereleri standart FastComments widget'ı ile aynı bileşen yapısını kullanır, bu nedenle uyguladığınız tüm genel özelleştirmeleri miras alır.

### Yerelleştirme

Collab Chat, standart FastComments widget'ı ile aynı yerelleştirme seçeneklerinin tamamını destekler. Kullanıcı arayüzü metnini farklı dillerde göstermek için `locale` seçeneğini ayarlayın:

[inline-code-attrs-start title = 'Yerel Ayarı Belirle'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // İspanyolca
});
[inline-code-end]

FastComments onlarca dili destekler. Locale ayarı, istemler, düğmeler ve yer tutucu metinler dahil tüm kullanıcı arayüzü metinlerini etkiler.

### Devralınan Özelleştirme Seçenekleri

Collab Chat, standart yorum widget'ını genişlettiği için temel widget'tan tüm özelleştirme seçeneklerini devralır. Buna özel CSS sınıfları, özel çeviriler, avatar özelleştirmesi, tarih biçimlendirme ve çok daha fazlası dahildir.

Mevcut tüm özelleştirme seçeneklerinin tam listesi için ana FastComments özelleştirme belgelerine bakın.

### Özel Yazı Tipleri ile Çalışma

Eğer siteniz özel yazı tipleri kullanıyorsa, Collab Chat kullanıcı arayüzü sayfanızın CSS'inden bu yazı tiplerini devralır. Bir widget özelleştirme kuralı oluşturmanız ve o kural içindeki özel CSS'e gerekli yazı tiplerini `@import` etmeniz gerekebilir eğer
yüzen sohbet penceresinin aynı yazı tiplerini kullanmasını istiyorsanız.

---