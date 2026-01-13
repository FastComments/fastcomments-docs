### Metin Seçimi Nasıl Çalışır

Kullanıcılar Collab Chat konteyneri içinde metin seçtiğinde, widget bu seçimi yakalar ve bir tartışma başlatmalarına izin verir. Seçim tek bir kelime kadar küçük veya farklı öğelere yayılmış birden çok paragraf kadar büyük olabilir.

### Seçim Gecikmesi

Masaüstü cihazlarda, bir kullanıcı metin seçtiği ile tartışma isteminin göründüğü zaman arasında 3.5 saniyelik bir gecikme vardır. Bu, kullanıcıların yalnızca metni kopyalamak veya okumak için vurgularken kullanıcı arayüzünün titremesini önler. Mobil cihazlarda, istem hemen görünür çünkü dokunmatik ekranlarda metin seçimi daha kasıtlıdır.

### Benzersiz Konuşma Kimlikleri

Her konuşma, sayfa URL'sini, DOM öğe yolunu ve serileştirilmiş metin aralığını birleştiren benzersiz bir `urlId` alır. Bu, her metin seçiminin daha sonra tekrar bulunabilecek ayrı bir konuşma oluşturmasını sağlar.

Yapılandırmanızda özel bir `urlId` sağlarsanız, nihai tanımlayıcıyı oluşturmak için bu, öğe yolu ve metin aralığı ile birleştirilecektir.

### Görsel Vurgular

Belirli bir metin seçimi için bir tartışma mevcut olduğunda, o metin görsel olarak vurgulanır. Vurgu, arka plan renkleri kullanılarak uygulanır ve üzerine gelindiğinde veya ilişkili sohbet penceresi açık olduğunda görünür.

Vurgulama sistemi, seçilen metni stillendirilebilen özel bir elementle sarmalayarak çalışır. Bu yaklaşım, alttaki HTML yapısı karmaşık olsa bile vurguların doğru kalmasını sağlar.

### Sohbet Penceresi Konumlandırması

Bir kullanıcı bir vurgunun üzerine tıkladığında veya yeni bir not oluşturduğunda, seçilen metnin yakınında bir sohbet penceresi görünür. Widget, bu pencere için mevcut görünüm alanı temelinde en iyi konumu otomatik olarak hesaplar.

Konumlandırma sistemi, sohbet penceresinin vurgudan hangi yönde genişleyeceğini belirtmek için `to-right`, `to-left`, `to-top` ve `to-bottom` gibi CSS sınıflarını kullanır. Mobil cihazlarda (768px'den dar ekranlarda), sohbet penceresi daha iyi kullanılabilirlik için her zaman tam ekran olarak görünür.

### Sohbet Penceresi Boyutları

Sohbet pencereleri masaüstünde 410px genişliğindedir, 20px boşluk ve vurgulanan metne işaret eden 16px görsel bir oka sahiptir. Bu boyutlar tutarlı davranışı sağlamak için sabittir, ancak görünümü CSS ile özelleştirebilirsiniz.

### Elemanlar Arası Seçimler

Kullanıcılar bir HTML öğesinin ortasından diğerinin başına kadar uzanan gibi birden çok HTML öğesini kapsayan metin seçebilirler. Aralık serileştirme sistemi bunu doğru şekilde işler ve öğe sınırları boyunca seçilen tüm metni vurgular.

### Tarayıcı Uyumluluğu

Metin seçimi sistemi, tüm modern tarayıcılarda desteklenen standart `window.getSelection()` API'sini kullanır. Eski Internet Explorer sürümleri için uyumluluk amacıyla `document.selection`'a geri dönüş yapılır.

### Seçimin Kalıcılığı

Bir metin seçimi için bir konuşma oluşturulduktan sonra, sayfa yeniden yüklense bile o açıklama (anotasyon) kalır. Serileştirilmiş aralık ve DOM yolu, kullanıcılar sayfaya geri döndüğünde widget'ın vurguları tam olarak aynı konumda geri yüklemesine olanak tanır.

Bu, sayfa içeriğiniz stabil kaldığı sürece güvenilir şekilde çalışır. Metin içeriğini değiştirir veya HTML yapınızı yeniden düzenlerseniz, mevcut açıklamalar metinle doğru şekilde hizalanmayabilir. Bu nedenle, aktif açıklamaları olan sayfalarda büyük içerik değişikliklerinden kaçınmak veya içerik değişiklikleri gerekli olduğunda açıklamaları taşımayı düşünmek en iyisidir.