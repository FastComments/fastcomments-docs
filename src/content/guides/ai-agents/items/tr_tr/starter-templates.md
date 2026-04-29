FastComments, baştan çalışan bir ajan yazmak zorunda kalmamanız için dört başlangıç şablonu sağlar. Bunlara [AI Ajanları sayfası](https://fastcomments.com/auth/my-account/ai-agents) üzerinden **Şablonlara Gözat**e tıklayarak ulaşılabilir.

When you pick a template:

1. Ajan, **Durum: Kuru Çalışma** ile ve şablona dayalı dahili bir adla oluşturulur (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`). Eğer bu ad tenant'ınızda alınmışsa, sonuna sayısal bir ek eklenir.
2. Her şey önceden doldurulmuş şekilde — prompt, tetikleyiciler, izin verilen eylemler ve varsa eşikler — doğrudan düzenleme formuna yönlendirilirsiniz. Üstte bir bant şu metni gösterir: "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Henüz hiçbir şey etkin değil. Ajan, kaydetmeden ve ya kuru çalışmayı açık tutarak (gözlemlemek için) ya da Durumu Etkin'e çevirene kadar işlem yapmaz.

### Dört şablon

- **[Moderatör](#template-moderator)** - yeni ve işaretlenen yorumları inceler, ilk kez kural ihlali yapanları uyarır, yalnızca bir uyarıdan sonra yasaklamaya yükseltir. Yeni yorumlar ve bayrak eşiği aşımlarında tetiklenir (varsayılan bayrak eşiği: 3). İzin verilen araçlar: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - ilk kez yorum yapanlara kısa, kişisel ve sıcak bir karşılama mesajı gönderir. `new-user-first-comment` tetiklendiğinde çalışır. İzin verilen araç: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - oy eşiğini aştıklarında önemli üst düzey yorumları sabitler (varsayılan: 10), önce daha önce sabitlenmiş yorumu kaldırır. Oy eşiği aşımlarında tetiklenir. İzin verilen araçlar: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - uzun konularda gecikme sonrası nötr, tek paragraflık bir özet gönderir, ardından onu sabitler. Özetlemeden önce konunun sakinleşmesi için yeni yorumlarda 30 dakikalık bir erteleme ile tetiklenir. İzin verilen araçlar: `write_comment`, `pin_comment`, `unpin_comment`.

### Bir şablonu özelleştirme

Şablonlar başlangıç noktalarıdır, sözleşme değildir. Beklenenler:

- Topluluğunuzun sesiyle eşleşmesi için **Başlangıç İstemini** (Initial prompt) dilediğiniz gibi ayarlayın.
- Ajanın ne sıklıkta çalışması gerektiğine göre **Tetikleyicileri** ekleyin veya kaldırın.
- Herhangi hassas bir eylem için **Onaylar** ekleyin - moderatör tarzı şablonlar için `ban_user` işlemini onay arkasına almak şiddetle tavsiye edilir.
- Ajanın yazılı politikanızı tutarlı şekilde uygulaması için **Topluluk yönergeleri** ekleyin. bkz. [Topluluk Yönergeleri](#community-guidelines).
- Beklediğiniz tetik sayısına uygun olarak her ajan için **Bütçeler** belirleyin.

Şablon, makul varsayılanları önceden dolduran bir başlangıç aracından ibarettir; kaydettikten sonra ajan size aittir.