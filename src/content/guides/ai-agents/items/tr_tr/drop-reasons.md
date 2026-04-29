Bir tetikçi çalıştığında ancak bir LLM çağrısıyla sonuçlanmadığında, platform bir nedenle bir "drop" kaydeder. Drop'lar [Analytics sayfası](#analytics-page) altında "Triggers skipped (this month)" bölümünde görünür.

### Drop nedenlerinin tam listesi

| Reason | What happened |
|---|---|
| `agentDaily` | Agent'ın günlük bütçe limiti aşıldı. |
| `agentMonthly` | Agent'ın aylık bütçe limiti aşıldı. |
| `tenantDaily` | Tenant'ın günlük bütçe limiti aşıldı. |
| `tenantMonthly` | Tenant'ın aylık bütçe limiti aşıldı. |
| `qps` | Agent'ın dakikalık başına düşen hız limiti (kaydırmalı 60s pencere) aşıldı. |
| `concurrency` | Agent'ın aynı anda çalışabilecek maksimum görev sayısı zaten doluydu. |

### Bu listede olmayanlar

Yönlendirme yoluna asla ulaşmayan bir tetikçi "drop" ile kaydedilmez — sadece gönderilmez. Buna şunlar dahildir:

- Agent **Devre Dışı**.  
- Tetikleyen yorum agent'ın [URL/locale kapsamına](#scope-url-locale) uymuyor.  
- Tetikleyen eylem aynı agent tarafından yapıldı (döngü önleme).  
- Tenant'ın faturalandırması geçersiz.  
- Agent, tenant'ın planında değil.

Bunlar sessiz atlamalardır, drop değil. Analytics'teki drops grafiğinde görünmezler.

### Analytics'te drop'ları okuma

[Analytics sayfası](#analytics-page) şunları gösterir:

- **Triggers skipped (this month)** - drop nedenine göre gruplanmış sayımlar.  
- **Agents at or near their cap** - hangi agent'ların limiti zorluyor olduğuna dair agent başına döküm; mevcut dönemde düşürülen tetikçi sayısı ile birlikte.

### Drop'ları gördüğünüzde ne yapmalısınız

- **`agentDaily` / `agentMonthly`** - agent'ın kendi limiti çok sıkı. Ya düzenleme formundan limiti yükseltin ya da agent'ı daraltın (URL/locale, daha sınırlı tetikleyiciler).  
- **`tenantDaily` / `tenantMonthly`** - hesap düzeyindeki limit çok sıkı. Tenant faturalama ayarlarından yükseltin veya harcamayı daha az sayıda agente dağıtın.  
- **`qps`** - trafik dakikalık kaydırmalı pencere sınırına çarpıyor. Genellikle viral bir dizinin agent'ın onları çalıştırabileceğinden daha hızlı tetikleyiciler yayması işaretidir. Agent'ın `maxTriggersPerMinute` ve `maxConcurrent` alanları bunu sınırlar; bunları yükseltmek verimi artırır fakat ani yük maliyetini de artırır.  
- **`concurrency`** - `qps` ile aynı temel sebep ancak uçuşta olan iş sayısı düzeyinde. Daha fazla paralellik gerekiyorsa `maxConcurrent` değerini yükseltin.

### Drop'lar vs hatalar

Bir drop "tetikçi hiç çalışmadı" demektir. Bir **hata** ise "tetikçi çalıştı fakat LLM çağrısı veya araç yönlendirmesi başarısız oldu" demektir. Hatalar [Run History](#run-history) sayfasında ayrı olarak takip edilir (durum `Error`).

### Drop'lar tekrar oynatmaları da durdurabilir

Aynı drop nedenleri uçuşta olan [test runs / replays](#test-runs-replays) işlemlerini de durdurur. Tekrar oynatma, hangi bütçenin aşıldığını belirten bir mesajla birlikte Error durumunda durur (örneğin, agent'ın günlük bütçesi).

### Döngü önleme kasıtlı olarak sessizdir

"Bu tetikçi başka bir agent'tan geldi ve döngüyü önlemek için atlandı" şeklinde bir drop nedeni yoktur. Bunu kaydetmek, yararlı bir sinyal sağlamadan analizleri karıştırır — tasarım gereği, agent fan-out'unun tetikçi patlamasıyla sonuçlanmaması gerekir. Eğer bastırılan bir döngünün olması gerektiği yerde bastırıldığından şüpheleniyorsanız, [Yorum Günlükleri](/guide-moderation.html#comment-logs) bölümünü kontrol edin - bot tarafından yazılan yorumlardaki `botId`, döngü kontrolünün anahtar aldığı değerdir.