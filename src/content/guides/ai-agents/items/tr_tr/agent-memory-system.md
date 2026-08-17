Agent hafızası, kiracı kapsamlı, **paylaşılan** bir anahtar‑değer havuzudur ve kiracınızdaki her ajan bunu okuyabilir ve üzerine yazabilir. Ajanların çalıştırmalar arasında bağlam taşıyabilmesi için vardır.

### Hafıza neden var

LLM bağlamı çalıştırma başına geçerlidir. Hafıza olmadan, bir kullanıcıya uyarı veren bir ajan, aynı kullanıcıyı bir sonraki sefer gördüğünde bu uyarıyı bilmenin bir yolu yoktur. Platformun yükseltme politikası - "yasaklamadan önce uyar" - ajanın önceki uyarıyı bulabilmesine dayanır. Hafıza, bunun çalışmasını sağlar.

### İki tür hafıza

- **WARNING** - [`warn_user`](#tool-warn-user) akışının bir parçası olarak otomatik olarak yazılır. Ajan, `WARNING` kayıtlarını elle yazmaz; bunlar bir kullanıcıyı uyarmanın yan etkisidir.
- **NOTE** - [`save_memory`](#tools-overview) tarafından yazılır. Ajanın gelecekteki ajanların bilmesini istediği genel amaçlı bağlam.

Yükseltme politikası, bir yasağın haklı olup olmadığını karar verirken özellikle `WARNING` kayıtlarına bakar.

### Kiracı kapsamlı, ajan paylaşımlı

Kiracınızdaki tüm ajanlar **tek bir hafıza havuzunu** paylaşır. Ajan A tarafından kaydedilen bir not, Ajan B'nin `search_memory` çağrılarında görülebilir. Bu kasıtlıdır - bir triage ajanın notları, moderatör ajanın kararlarını bilgilendirmelidir.

`tenantId`, yürütücü tarafından ajanın kendi kiracısından ayarlanır - LLM argümanlarından asla - bu yüzden kiracılar arası hafıza sızıntıları yapı itibarıyla imkânsızdır.

### Bir hafıza kaydında neler var

Her hafıza girişi şunları içerir:

- **Hangi ajan** tarafından yazıldığı ve ne zaman.
- **Kiminle ilgili olduğu** - bu hafızanın tanımladığı kullanıcı. Ajan bunu uyduramaz; platform, ajanın neyle tetiklendiğine bağlı olarak otomatik olarak doldurur.
- **Gizli bir alt‑hesap sinyali** - platform ayrıca (özel olarak) orijinal yorumun IP parmak izini kaydeder, böylece gelecekteki hafıza aramaları aynı IP'den gönderi yapan diğer hesapların notlarını ortaya çıkarabilir. Parmak izi ajana veya LLM'ye asla gösterilmez.
- **Notun kendisi** - 2000 karaktere kadar serbest metin.
- **Etiketler** - geri getirme için, en fazla 10 kısa etiket.
- **Bir tür** - ya bir uyarı ya da genel bir not.
- **İsteğe bağlı bir yorum bağlantısı** - hafıza belirli bir yorumla ilişkilendirilmişse.

### Arama davranışı

[`search_memory`](#tools-overview) en fazla 25 kayıt döndürür, en yeni önce sıralanır ve otomatik olarak (tetikleyicinin kullanıcısına) VEYA (tetikleyicinin IP'sindeki diğer hesaplara) kapsamlandırılır. Sonuçlar ayrıca tüm döndürülen içeriklerde toplam 8000 karakterle sınırlıdır – limit aşıldığında daha eski girişler atılır.

Ajan, `userId` veya `targetIpHash` parametresini geçmez. Her ikisi de yürütücü tarafından ayarlanır.

### Kalıcılık

Hafızanın **TTL'si yoktur**. Kayıtlar, açıkça kaldırılana kadar kalıcıdır. Bir kullanıcıyla ilgili WARNING kayıtlar kasıtlı olarak asla otomatik silinmez – yükseltme geçmişi süresiz bulunabilir olmalı, aksi takdirde platformun "yasaklamadan önce arama" kontrolü anlamsızdır.

Hafızanın kaldırılmasının üç yolu:

- Bir moderatör, temel yorumu siler – o yorumla ilişkili tüm hafıza kademeli olarak silinir.
- Bir kullanıcı silinir – o kullanıcıyla ilgili tüm hafıza girişleri aynı işlemde kaldırılır.
- Kiracınız silinir.

Bugün, bireysel hafıza kayıtlarını silmek için bir yönetici UI'si yoktur.

### Kuru çalıştırmada hafıza

Kuru çalıştırma ajanları hafıza **yazmaz**. Bu tasarım gereğidir: bir kuru çalıştırma ajanın varsayımsal kararları paylaşılan hafıza havuzunu kirletmemelidir. `search_memory` aracılığıyla geri okuma, kuru çalıştırmada normal şekilde çalışır – ajan, canlı ajanların gerçek hafızalarını görebilir – ancak onlara ekleyemez.

### Tekrar çalıştırmalarda hafıza

Kuru çalıştırma gibi: tekrar (replay) ajanları hafıza yazmaz. Tekrarlar sadece ön izleme amaçlıdır. Bkz. [Test Runs (Replays)](#test-runs-replays).

### Kısıtlamalar özeti

| Sınır | Değer |
|---|---|
| Hafıza içeriği maksimum uzunluğu | 2000 chars |
| Hafıza etiketi maksimum uzunluğu | 64 chars |
| Hafıza etiketleri maksimum sayısı | 10 |
| Hafıza sorgusu maksimum uzunluğu | 200 chars |
| Hafıza arama sonuç limiti | 25 records |
| Hafıza arama toplam içerik sınırı | 8000 chars |

### Ayrıca bakınız

- [Tool: save_memory](#tools-overview) yazmak için.
- [Tool: search_memory](#tools-overview) okumak için.
- [Tool: warn_user](#tool-warn-user) - WARNING türü hafıza yazan tek araç.
- [Tool: ban_user](#tool-ban-user) - sistem istemi, bundan önce `search_memory` çağrılmasını gerektirir.