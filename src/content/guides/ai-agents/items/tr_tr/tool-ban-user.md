---
Ban aracı, bir ajanın çağırabileceği en ciddi işlemdir. Topluluğunuzdan bir kullanıcıyı belirli bir süreyle yasaklar ve birkaç seçenek sunar.

### Ne yapar

Ajan altı süreden birini seçer:

- Bir saat
- Bir gün
- Bir hafta
- Bir ay
- Altı ay
- Bir yıl

Ayrıca **görünür yasak** (kullanıcı açık bir yasak mesajı görür ve itiraz edebilir) ile **gölge yasağı** (kullanıcı paylaşım yapmaya devam edebilir ancak içeriği diğer kullanıcılardan gizlenir) arasında seçim yapar. Platformun yönergeleri, ilk kez veya sınırda olan vakalarda görünür yasakları; açıkça kötü niyetli tekrar eden ihlallerde ise gölge yasakları tercih etmesini söyler.

### İki yıkıcı alt-seçenek

İki ek seçenek varsayılan olarak ajandan **gizlidir**. Her iki seçeneği etkinleştirmek için, ajan düzenleme formundaki **Ban options** bölümünde ilgili onay kutusunu işaretleyin:

- **Kullanıcının tüm yorumlarını silmeye izin ver.** Etkinleştirildiğinde, ajan yasaklanan kullanıcının tenant'ınızda şimdiye kadar gönderdiği tüm yorumları silmeyi de seçebilir. Mevcut içeriğin değeri olmadığı açık spam, doxxing veya koordineli taciz durumları için ayırın. **Yıkıcı ve geri döndürülemez.**
- **IP ile yasaklamaya izin ver.** Etkinleştirildiğinde, ajan yorumun gönderildiği IP'yi de yasaklamayı seçebilir. Alt hesaplarla yasak atlatmaya karşı etkili olabilir. **Paylaşılan IP'ler için kaçının** (kurumsal, okul, mobil operatörler) - aynı ağdaki masum kullanıcılar engellenecektir.

Platform bunları sunucu tarafında da kısıtlar: ajan kötüye düşse ve seçeneği çağırmaya çalışsa bile, siz izin vermediğiniz sürece istek reddedilir.

### Yükseltme politikası

Yasaklamadan önce, platform ajanı yönlendirir:

1. Kullanıcıyla ilgili önceki uyarılar veya notlar için [ajan belleği](#agent-memory-system) içinde arama yapın.
2. İlk ihlallerde kullanıcıyı yasaklamaktan ziyade [uyarmayı](#tool-warn-user) tercih edin.
3. Uyarı adımını yalnızca açıkça ağır vakalar (yasal olmayan içerik, doxxing, koordineli spam) için atlayın - ve gerekçesinde nedenini açıklayın.

Bu politika ajan talimatlarında yer alır, sert bir sunucu tarafı kuralı değildir; bu nedenle **yasaklamaları onaya bağlamak şiddetle tavsiye edilir**.

### AB bölgesi: insan onayı gerekli

AB bölgesinde, bu araç Dijital Hizmetler Yasası'nın 17. maddesi gereği **onay için kilitlidir**. AB bölgesi bir tenant üzerindeki herhangi bir ajanın gerçekleştirdiği her yasak, insan denetimi için [onaylar gelen kutusu](#approval-workflow)'na düşer. Bakınız: [AB DSA Madde 17 Uyumluluğu](#eu-dsa-compliance).

### Öneriler

- Her yerde en az ilk ay için onay gerektirin.
- Etkinleştirirseniz **delete-all-comments** seçeneğini her zaman onaya bağlayın - bu geri döndürülemez.
- Ajan güven kazandıktan sonra bile **IP ban** seçeneğini onaya bağlamayı düşünün - paylaşılan bir ağda yapılan IP yasağının maliyeti ajanın çalışma geçmişinde görünmez.

### Ayrıca bakınız

- Platform genelinde yasaklamaların nasıl çalıştığı hakkında moderasyon kılavuzundaki [Kullanıcıları Yasaklama](/guide-moderation.html#banning-users) ve [Joker Karakterlerle Kullanıcıları Yasaklama](/guide-moderation.html#banning-users-wildcards) bölümlerine bakın.
- [Kullanıcıyı Uyarmak](#tool-warn-user) - daha hafif bir yükseltme adımı.

---