The **Topluluk yönergeleri** alanı düzenleme formunda isteğe bağlı bir politika metni bloğudur ve bu ajan için her çalıştırmada kullanıcı-rolü bağlam mesajına eklenir. Bu blok, platformun yorum gövdelerine ve diğer kullanıcı tarafından sağlanan içeriğe uyguladığıyla aynı şekilde güvensiz metin olarak sınırlandırıldığı için model bunu sistem talimatı olarak değil politika referansı olarak ele alır. Bu, "bu sitede hangi davranışların izinli ve hangi davranışların izinli olmadığı"nı yazmak için kanonik yerdir, böylece ajan bunu tutarlı şekilde uygular.

### Başlangıç isteminden nasıl farklıdır

- **Başlangıç istemi** - ajanın rolü ve karar verme tarzı. "Sen bir moderatörsün. Yasaklamadan önce uyarmayı tercih et."
- **Topluluk yönergeleri** - topluluğunuzun kuralları, politika diliyle. "Kişisel saldırı yok. 24 saatten daha kısa süredir var olan hesaplardan tanıtıcı bağlantılara izin yok. Bir konu kızışmışsa konuyla alakasız yorumlar kaldırılabilir."

Her ikisi de aynı bağlam penceresine akar, ancak farklı katmanlarda girerler - başlangıç istemi sistem rolünün bir parçasıdır, yönergeler belgesi ise kullanıcı-rolü bağlam mesajı içinde sınırlandırılmış metindir. Bu ayrım, birini güncellemek istediğinizde diğerini tekrar okumadan düzenlemeyi kolaylaştırır.

### İyi bir yönergeler belgesi nasıl olmalı

Kısa, spesifik, bir insan tarafından yazılmış bir belge. Liste halinde olması düzyazıdan daha etkilidir:

[inline-code-attrs-start title = 'Topluluk Yönergeleri Örneği'; type='text' inline-code-attrs-end]
[inline-code-start]
İzin verilenler:
- Özlü itirazlar, hatta sert bir üslupla ifade edilmiş olsa bile.
- Orijinal kaynaklara bağlantılar, yeni hesaplardan gelse bile.
- Ana konu izin veriyorsa, konuyla alakasız yan notlar.

İzin verilmeyenler:
- Belirli isimleri hedef alan kişisel saldırılar.
- Doxxing veya özel bilgilerin paylaşılması.
- Koordineli tanıtım faaliyetleri (aynı dış bağlantıyı birden fazla yorumla öne çıkarma).
- Tartışmayı saptırmak için yapılan yorumlar.

Sınırda olanlar:
- Hedefi olmayan sert dil. Bir kişiye yöneltilmemişse izin verilir.
- Sayfanın konusu dışında siyasi konular. Konuyla alakasız; önce uyarın, ısrarcı olmadıkça kaldırmayın.
[inline-code-end]

Ajan bunu her çalıştırmada uygular. Yönergeleri değiştirirseniz, değişiklik bir sonraki tetiklemede yürürlüğe girer - geçmiş çalıştırmalar geriye dönük olarak yeniden değerlendirilmez.

### Buraya ne koymamalısınız

- **Çıktı biçimlendirme talimatları** ("HTML ile cevap ver", "emoji kullan"). Bunlar [başlangıç istemi](#personality-prompt) içinde yer almalıdır.
- **Yerelleştirilmiş metin.** Yönergeler belgesi, istem gibi, aynı nedenle **yalnızca İngilizce** olmalıdır - makine çevirisi ajan davranışını sessizce değiştirebilir. Bölgeye göre değişen politikalarınız varsa, hepsini bu tek belgede İngilizce yazın ve belgeyi "Almanca dilindeki sayfalar için: ..." şeklinde yapılandırın.
- **Dış politikaların uzun alıntıları.** Parafraz edin. Uzun bağlam her çalıştırmada token maliyeti doğurur.
- **PII veya sırlar.** Bu metin her çalıştırmada LLM sağlayıcısına gönderilir.

### Uzunluk

Alan **4000 karakter** ile sınırlandırılmıştır (hem form hem de kaydetme rotası tarafından zorlanır). Her çalıştırmadaki token maliyeti uzunluğa orantılıdır, bu nedenle sınır içinde bile birkaç yüz kelime genellikle yeterlidir. Topluluk politikalarınız birçok sayfaya yayılıyorsa, ajan için bu alana özgü özetlenmiş bir spesifikasyon hazırlayın.

### Sürümleme

Yönergeler belgesi için yerleşik bir sürüm geçmişi yoktur - ajan en son kaydedilen değeri kullanır. Geçmiş istiyorsanız, her büyük düzenlemeden önce belgeyi kendi takip sisteminize kopyalayın. [İstemleri Geliştirme](#refining-prompts) akışı *başlangıç istemi* için yapılan değişiklikleri kaydedebilir ancak yönergeler belgesini sürümlendirmez.

---