FastComments, sıfırdan çalışan bir ajan yazmak zorunda kalmamanız için beş başlangıç şablonuyla birlikte gelir. Bunlara [AI Agents sayfasından](https://fastcomments.com/auth/my-account/ai-agents) **Browse templates** seçeneğine tıklayarak ulaşabilirsiniz.

Bir şablon seçtiğinizde:

1. Ajan, **Status: Dry Run** durumunda ve şablona dayalı bir dahili adla oluşturulur (`tos_enforcer`, `welcome_greeter`, `top_comment_pinner`, `thread_summarizer`, `gaslight_detector`). Bu ad kiracınızda (tenant) zaten alınmışsa, sayısal bir son ek eklenir.
2. Doğrudan her şey önceden doldurulmuş düzenleme formuna yönlendirilirsiniz - prompt, tetikleyiciler, izin verilen eylemler ve varsa eşik değerleri. Üstte bir banner şu metni gösterir: "Created from the {templateName} template. Review the settings below, then change status to Enabled when you're ready."
3. Henüz hiçbir şey etkinleştirilmedi. Ajan, kaydedene ve ya dry-run modunda bırakana (gözlemlemek için) ya da Enabled durumuna geçene kadar işlem yapmaz.

### Beş şablon

- **[Moderator](#template-moderator)** - yeni ve işaretlenmiş yorumları inceler, ilk kez kural ihlali yapanları uyarır, yasaklamaya yalnızca bir uyarıdan sonra geçer. Yeni yorumlarda ve işaret eşiği aşımlarında tetiklenir (varsayılan işaret eşiği: 3). İzin verilen araçlar: `mark_comment_approved`, `mark_comment_spam`, `warn_user`, `ban_user`.

- **[Welcome Greeter](#template-welcome-greeter)** - ilk kez yorum yapanlara kısa, kişisel ve sıcak bir karşılama ile yanıt verir. Yeni kullanıcının ilk yorumunda tetiklenir. İzin verilen araç: `write_comment`.

- **[Top Comment Pinner](#template-top-comment-pinner)** - üst düzey, içerikli yorumlar bir oy eşiğini aştığında (varsayılan: 10) sabitler, önceden sabitlenmiş yorumu önce kaldırır. Oy eşiği aşımlarında tetiklenir. İzin verilen araçlar: `pin_comment`, `unpin_comment`.

- **[Thread Summarizer](#template-thread-summarizer)** - uzun tartışmalara gecikme sonrasında tarafsız, tek paragraflık bir özet gönderir ve ardından sabitler. Yeni yorumlarda tetiklenir ve tartışmanın özetlemeden önce oturması için 30 dakikalık bir gecikme kullanır. İzin verilen araçlar: `write_comment`, `pin_comment`, `unpin_comment`.

- **[Gaslight Detector](#template-gaslight-detector)** - yanıtları çarpıtan, tartışma ortasındaki yeniden yazmalar için yorum düzenlemelerini izler, orijinal metni geri yükler ve yazara DM gönderir. Yorum düzenlemelerinde tetiklenir. İzin verilen araçlar: `edit_comment`, `warn_user`, `send_dm`.

### Bir şablonu özelleştirme

Şablonlar başlangıç noktalarıdır, sözleşme değil. Şunları yapmanız beklenir:

- **Initial prompt** alanını topluluğunuzun sesine uyacak şekilde ayarlayın.
- Ajanın ne sıklıkla çalışması gerektiğine uygun şekilde **Triggers** ekleyin veya kaldırın.
- Herhangi bir hassas eylem için **Approvals** ekleyin - moderatör tarzı şablonlarda `ban_user` işlemini onay arkasına almanızı şiddetle tavsiye ederiz.
- Ajanın yazılı politikanızı tutarlı bir şekilde uygulaması için **Community guidelines** ekleyin. Bkz. [Community Guidelines](#community-guidelines).
- Beklediğiniz tetikleyici sayısına uygun, ajana özel **Budgets** belirleyin.

Şablon, yalnızca makul varsayılanları önceden dolduran bir araçtır; kaydedildikten sonra ajan sizindir.