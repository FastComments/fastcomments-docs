Varsayılan olarak bir ajan tetikleyici çalıştıktan sonra **hemen** çalışır. Düzenleme formundaki **Çalıştırmadan önceki gecikme** alanı bunu değiştirir: platform tetikleyiciyi sıraya alır ve ajanı planlanan zamanda çalıştırır.

### Gecikme ne zaman kullanılmalı

- **Bayrak-eşiği tetikleyicileri** - bayraklar genellikle toplu halde gelir. 10–30 dakikalık bir gecikme tablonun oturmasına izin verir, böylece ajan geliş anındaki yerine son bayrak sayısına göre hareket eder.
- **Oy-eşiği tetikleyicileri** - aynı mantık, özellikle aşağı oy saldırıları için.
- **Konuşma özetleme** - [Konu Özetleyici şablonu](#template-thread-summarizer) varsayılan olarak 30 dakikalık bir gecikme kullanır, böylece iki yanıtlık bir konu yerine gelişmesi için zamanı olmuş bir tartışmayı özetler.
- **Soğuma / yeniden değerlendirme** - "Bir yorum kilitlendikten 24 saat sonra, kilidi açıp açmamayı değerlendirin."

### Yapılandırma

- **Alan**: Çalıştırmadan önceki gecikme.
- **Aralık**: 0 ile 2,592,000 saniye (30 gün).
- **Birimler**: Saniye, dakika, saat, veya gün.

### İdempotans

Ertelenmiş kuyruk tetikleyicileri tekilleştirmez. 30 dakikalık gecikmeye sahip bir ajana 1 saniye arayla gelen iki bayrak, her ikisi de 30 dakika sonra bir çalışmayı zamanlayacaktır ve ajan **iki kez** çalışacaktır; her defasında (çoğunlukla) aynı bağlam üzerinde. Eğer pencere başına en fazla bir çalıştırma semantiği istiyorsanız, ajan bunu kendisi uygulamalıdır — genellikle ilk çalıştırmada bir [hafıza notu](#tools-overview) yazarak ve sonraki çalıştırmalarda bunu kontrol ederek.

### Maliyet notu

Ertelenmiş tetikleyiciler çalıştırılmadan **önce** kaydedilir. Yüksek gecikmeli bir ajandaki tetikleme patlaması kuyruğa token harcamadan yığılabilir; maliyet yalnızca cron onları gönderdiğinde ödenir. Ertelenmiş tetikleyicilerin ne sıklıkla gerçekten çalıştırıldığını veya bütçe nedeniyle çalışma zamanında düşürüldüğünü görmek için [Çalıştırma Geçmişi](#run-history) ve [Atılma Nedenleri](#drop-reasons) kullanın.

### Tekrar oynatma gecikmeyi dikkate almaz

[Test Runs (Replays)](#test-runs-replays) özelliği ajanı geçmiş yorumlara karşı hemen çalıştırır — yapılandırılmış gecikmeyi beklemez. Bunu bir özellik olarak değerlendirin: tekrar oynatmalar ajanın verilen bağlamda **ne yapacağını** önizlemek içindir, gerçek zamanlı zamanlamayı yeniden üretmek için değil.