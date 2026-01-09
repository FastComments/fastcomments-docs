### Yüzde Tabanlı Konumlandırma

Image Chat, görüntü üzerindeki sohbet işaretlerini konumlandırmak için piksel koordinatları yerine yüzde tabanlı koordinatları kullanır. Bir kullanıcı bir görüntüye tıkladığında, widget tıklamanın piksel koordinatlarını görüntü genişliğinin ve yüksekliğinin yüzdelerine dönüştürür. Bu yaklaşım, görüntü nasıl gösterilirse gösterilsin işaretlerin doğru konumda kalmasını sağlar.

Örneğin, bir kullanıcı 1000px genişliğindeki bir görüntünün sol kenarından 250 piksel uzaklığa tıklarsa, widget bunu sol kenardan %25 olarak kaydeder. Başka bir kullanıcı aynı görüntüyü mobil cihazda 500px genişlikte görüntülediğinde, işaret sol kenardan 125 pikselde görünür ki bu hâlâ genişliğin %25'idir.

### Duyarlı Düzenler için Faydalar

Bu yüzde sistemi Image Chat'in tüm cihaz boyutlarında ve yönlerinde sorunsuz çalışmasını sağlar. Görüntüleriniz ekran genişliğine, CSS kurallarına veya kapsayıcı kısıtlamalarına bağlı olarak farklı boyutlarda gösterilebilir, ancak işaretler her zaman hedeflenen konumlarla doğru şekilde hizalanır.

Büyük monitörlü masaüstü bilgisayarlardaki kullanıcılar, küçük ekranlı akıllı telefonlardaki kullanıcılarla aynı göreli konumlarda işaretleri görür. İşaretler, görüntünün kendisiyle orantılı olarak ölçeklenir.

### Görüntü Ölçeklendirme ve En-Boy Oranı

Görüntünüz ölçeklenirken en-boy oranını koruduğu sürece (ki bu varsayılan tarayıcı davranışıdır), yüzde tabanlı işaretler mükemmel şekilde hizalanmış kalır. Widget, görüntülerin orantılı olarak ölçeklendiğini varsayar ve konumları bu varsayıma göre hesaplar.

Eğer görüntünün en-boy oranını bozan CSS uygularsanız (örneğin belirli boyutlarla `object-fit: cover` kullanmak gibi), işaret konumları doğru hizalanmayabilir. En iyi sonuçlar için görüntülerin doğal olarak ölçeklenmesine izin verin veya en-boy oranını korumak için `object-fit: contain` kullanın.

### Sohbet Karesi Boyutlandırması

Sohbet işaretlerinin görsel boyutu da yüzde tabanlıdır. `chatSquarePercentage` yapılandırma seçeneğinin varsayılanı %5'tir; bu, her karenin görüntü genişliğinin %5'i olduğu anlamına gelir. Bu, farklı görüntü boyutlarında tutarlı bir görsel yoğunluk sağlar.

Varsayılan %5 ayarıyla 1000px genişliğindeki bir görüntüde işaretler 50px kare olur. 500px genişliğindeki bir görüntüde aynı işaretler 25px kare olur. Bunlar görüntü boyutuna orantılı kalır.

### Mobil Davranış

768px'den daha dar ekranlarda Image Chat mobil için optimize edilmiş bir düzene geçer. Sohbet pencereleri markerın yanında kayan pencere yerine tam ekran açılır. Bu, kayan pencerelerin görüntünün çok büyük kısmını kapatacağı küçük ekranlarda daha iyi kullanılabilirlik sağlar.

İşaretlerin kendileri yüzde tabanlı konumlarında görünür ve tıklanabilir kalır. Kullanıcılar tüm tartışmaların nerede olduğunu görebilir ve tam ekran sohbet arayüzünü açmak için işaretlere dokunabilir.

### Dinamik Görüntü Yükleme

Yüzde tabanlı sistem, görüntüler asenkron olarak yüklendiğinde veya sayfa yüklendikten sonra boyutları değiştiğinde bile doğru çalışır. Widget, görüntü öğesini izler ve görüntü boyutları değiştiğinde işaret konumlarını yeniden hesaplar.

Eğer görüntüleri tembel yükleme (lazy-loading) yapıyorsanız veya farklı kırılma noktalarında farklı boyutlara sahip duyarlı görüntüler (responsive images) uyguluyorsanız, görüntü boyutu değiştiğinde işaretler otomatik olarak ayarlanır.

### Cihazlar Arası Tutarlılık

Koordinatlar veritabanında yüzde olarak saklandığı için, bir masaüstü bilgisayarda oluşturulan tartışma tablet veya telefonda görüntülendiğinde aynı göreli konumda görünür. Kullanıcılar cihazlar arasında konumlandırma tutarsızlığı olmadan işbirliği yapabilir.

Bu iki yönlü olarak çalışır. Mobil cihazda belirli bir noktaya dokunarak oluşturulan bir tartışma, büyük bir masaüstü monitörde görüntülendiğinde aynı göreli konumda görünür.

### Görünüm Penceresi (Viewport) Hususları

Widget yüzdeleri görünüm penceresi yerine görüntü öğesinin kendisine göre hesaplar. Bu, sayfayı kaydırmanın veya tarayıcı pencere boyutunu değiştirmenin işaret konumlarını etkilemediği anlamına gelir. Görünüm penceresi değişikliklerine bakılmaksızın işaretler görüntü üzerindeki konumlarına sabitlenmiş kalır.

### İçeriği Geleceğe Hazırlama

Yüzde tabanlı yaklaşım, görüntü tartışmalarınızı düzen, tasarım veya cihaz ekosistemindeki değişikliklere karşı dayanıklı kılar. Yeni ekran boyutları ve cihazlar ortaya çıktıkça, mevcut tartışmalar herhangi bir güncelleme veya taşıma gerektirmeden doğru şekilde görüntülenmeye devam edecektir.