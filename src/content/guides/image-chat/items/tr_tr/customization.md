### Karanlık Mod Desteği

Image Chat yerleşik karanlık mod desteği içerir. Yapılandırmanızda `hasDarkBackground: true` olarak ayarladığınızda, sohbet pencereleri ve UI öğeleri koyu arka planlarda iyi görünmeleri için otomatik olarak uyum sağlar.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

Karanlık mod stil uygulaması sohbet pencerelerine, işaretçi karelerine ve tüm etkileşimli öğelere uygulanır. Sitenizde bir karanlık mod seçeneği varsa, mod değiştiğinde widget'ı yeniden başlatabilir veya aşağıda açıklanan body sınıfı yaklaşımını kullanabilirsiniz.

### Dinamik Karanlık Mod

Sitenizin karanlık modu body öğesine `.dark` sınıfı ekleyerek kontrol ediliyorsa, Image Chat UI bunu yeniden başlatma gerektirmeden otomatik olarak dikkate alır. Widget'ın stilleri bu sınıfın varlığına yanıt verecek şekilde tasarlanmıştır.

```css
/* Karanlık mod CSS'iniz */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Özel Stil ile CSS

İşaretçilerin, sohbet pencerelerinin ve diğer öğelerin görünümünü CSS ile özelleştirebilirsiniz. Widget, stil sayfanızda hedefleyebileceğiniz belirli sınıflar ekler.

Sohbet kareleri ve pencereleri FastComments yorum balonu stil sistemi kullanır, bu nedenle standart yorum widget'ına uyguladığınız herhangi bir özelleştirme Image Chat'i de etkiler.

### Sohbet Karesi Boyutlandırma

`chatSquarePercentage` seçeneği tıklanabilir işaretçilerin boyutunu kontrol eder. Varsayılan, görüntü genişliğinin %5'idir:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Daha büyük, daha görünür kareler
});
```

Daha küçük değerler görüntüye daha sinsi şekilde karışan işaretçiler oluşturur. Daha büyük değerler, özellikle mobil cihazlarda veya erişilebilirlik amaçları için işaretçileri daha belirgin ve tıklanması daha kolay hale getirir.

### Mobil Davranış

768px'den dar ekranlarda, Image Chat otomatik olarak mobil için optimize edilmiş bir düzen kullanır. Sohbet pencereleri işaretçilerin yanında kayan şekilde görünmek yerine tam ekran olarak görünür, küçük ekranlarda daha iyi kullanılabilirlik sağlar.

İşaretçiler görüntü üzerindeki duyarlı konumlarında görünür kalır. Kullanıcılar herhangi bir işaretçiye dokunarak tam ekran sohbet arayüzünü açabilir. Bu davranış yerleşiktir ve herhangi bir yapılandırma gerektirmez.

### Sohbet Penceresi Görünümü

Sohbet pencereleri masaüstünde 300px genişliğindedir ve işaretçiye işaret eden 16px'lik bir ok bulunur. Pencereler, `to-right`, `to-left`, `to-top` ve `to-bottom` gibi konumlandırma sınıflarını kullanarak mevcut görünüm alanına (viewport) göre otomatik olarak konumlanır.

Bu pencerelerin renklerini, yazı tiplerini, boşluklarını veya diğer görsel özelliklerini ayarlamak için özel CSS ekleyebilirsiniz. Sohbet pencereleri standart FastComments widget'ı ile aynı bileşen yapısını kullandığından, uyguladığınız herhangi bir genel özelleştirmeyi devralır.

### Tembel Başlatma

Sohbet pencereleri masaüstü kullanıcıları için hover ile veya oluşturulduğunda hemen başlatılır. Bu, kullanıcılar gerçekten bir işaretçi ile etkileşime girene kadar sohbet arayüzünü yalnızca render ederek başlangıç yükünü azaltır.

Tembel başlatma şeffaf bir şekilde gerçekleşir. Kullanıcılar herhangi bir gecikme fark etmez, ancak tarayıcının bir görüntüde birden çok işaretçi varsa onlarca gizli sohbet penceresini render etmesine gerek kalmaz.

### Yerelleştirme

Image Chat, standart FastComments widget'ı ile aynı yerelleştirme seçeneklerinin tümünü destekler. UI metinlerini farklı dillerde göstermek için `locale` seçeneğini ayarlayın:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Fransızca
});
```

FastComments onlarca dili destekler. Locale ayarı istemler, düğmeler ve yer tutucu metinler dahil tüm UI metinlerini etkiler.

### Devralınan Özelleştirme Seçenekleri

Image Chat, standart yorum widget'ını genişlettiğinden, temel widget'tan tüm özelleştirme seçeneklerini devralır. Bu, özel CSS sınıfları, özel çeviriler, avatar özelleştirmesi, tarih formatlama ve çok daha fazlasını içerir.

Mevcut tüm özelleştirme seçenekleri listesi için ana FastComments özelleştirme belgelerine bakın.

### Özel Yazı Tipleriyle Çalışma

Siteniz özel yazı tipleri kullanıyorsa, Image Chat UI bu yazı tiplerini sayfanızın CSS'inden devralır. Sohbet pencereleri sayfanızın DOM'u içinde render edilir ve mevcut tipografi ayarlarınızı dikkate alır.

En iyi sonuçlar için özel yazı tiplerinizin Image Chat'i başlatmadan önce yüklendiğinden emin olun veya yazı tipleri yüklenene kadar kısa bir sürede biçimsiz metin yanıp sönmesi olabileceğini kabul edin.

### İşaretçi Görsel Tasarımı

Kare işaretçiler, görüntüyü bunaltmadan fark edilmelerini sağlayan ince bir görsel tasarıma sahiptir. Farklı bir görsel işlem istiyorsanız görünümünü CSS ile özelleştirebilirsiniz.

İşaretçiler, kullanıcı fareyi üzerlerine getirdiğinde geri bildirim sağlayan hover durumlarını içerir. Dokunmatik cihazlarda, dokunma etkileşimi sohbet penceresini açarak anında geri bildirim sağlar.