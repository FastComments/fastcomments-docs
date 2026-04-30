FastComments beş başlangıç şablonu ile gelir, böylece çalışan bir ajanı sıfırdan yazmak zorunda kalmazsınız. Bunlara [AI Agents page](https://fastcomments.com/auth/my-account/ai-agents) üzerinden **Browse templates**e tıklayarak ulaşabilirsiniz.

When you pick a template:

1. Ajan **Status: Dry Run** ile ve şablona dayalı dahili bir adla oluşturulur (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Eğer o ad tenant'ınızda zaten alınmışsa sayısal bir son ek eklenir.
2. Düzenleme formuna doğrudan yönlendirilirsiniz; her şey - prompt, tetikleyiciler, izin verilen eylemler ve varsa eşik değerleri - önceden doldurulmuştur. Üst kısımda yer alan bir banner şu metni gösterir: "{templateName} şablonundan oluşturuldu. Aşağıdaki ayarları gözden geçirin, hazır olduğunuzda durumunu Enabled olarak değiştirin."
3. Henüz hiçbir şey etkin değildir. Ajan, kaydetmeyene kadar veya dry-run açıkken (gözlemlemek için) bırakana kadar ya da Enabled konumuna çevrilene kadar işlem yapmaz.

### The five templates

- **[Moderator](#template-moderator)** - yeni ve işaretlenmiş yorumları inceler, ilk kez kural ihlali yapanları uyarır, yalnızca bir uyarıdan sonra yasaklamaya yükseltir. Tetiklenir: new comments and on flag-threshold crossings (default flag threshold: 3). İzin verilen araçlar: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - ilk kez yorum yapanlara kısa, kişisel bir karşılama ile sıcak bir şekilde yanıt verir. Tetiklenir: new-user-first-comment. İzin verilen araç: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - anlamlı üst seviye yorumları oy eşiğini aştıklarında (varsayılan: 10) sabitler; önceki sabitlenmiş yorumu önce sabitsizleştirir. Tetiklenir: vote-threshold crossings. İzin verilen araçlar: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - uzun başlıklar için gecikmeli olarak nötr, tek paragraflık bir özet gönderir ve ardından bunu sabitler. Tetiklenir: new comments with a 30-minute deferral. İzin verilen araçlar: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - yanıtları çarpıtan ortadaki başlık yeniden yazımlarını izler, orijinal metni geri yükler ve yazara DM gönderir. Tetiklenir: comment edits. İzin verilen araçlar: `edit_comment`, `warn_user`, `send_dm`.

### Customizing a template

Şablonlar başlangıç noktalarıdır, sözleşme değildir. Sizden beklenenler:

- **Initial prompt**u topluluğunuzun sesine uyacak şekilde ayarlamak.
- Ajanın ne sıklıkla çalışması gerektiğine uyan **Triggers** eklemek veya kaldırmak.
- Herhangi bir hassas eylem için **Approvals** eklemek - moderator tarzı şablonlar için `ban_user` eylemini onay arkasına almak şiddetle önerilir.
- Ajanın yazılı politikanızı tutarlı şekilde uygulaması için **Community guidelines** eklemek. Bkz. [Community Guidelines](#community-guidelines).
- Beklediğiniz tetikleme sayısına uygun per-ajans **Budgets** ayarlamak.

Şablon sadece makul varsayılanları önceden dolduran bir taşıttır; kaydettikten sonra ajan size ait olur.