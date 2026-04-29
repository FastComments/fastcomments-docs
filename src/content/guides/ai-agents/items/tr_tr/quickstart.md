Bu, “Yapay Zeka Ajanlarımız var” durumundan “bir ajan onaylara tabi olarak canlı trafiğe yanıt veriyor” durumuna beş dakikalık yoldur. Uzun formu istiyorsanız, her adım bunu derinlemesine ele alan sayfaya bağlantı verir.

### 1. AI Ajanları sayfasını açın

Hesabınızdaki [AI Agents](https://fastcomments.com/auth/my-account/ai-agents) sayfasına gidin. Buraya ilk kez geldiğinizde ya şunu göreceksiniz:

- Bir **Browse templates** ve **Start from scratch** düğmesiyle boş durum (oluşturabileceğiniz ajanlar mevcut), veya
- Planınız ajanları içermiyorsa bir satış sayfası - bkz. [Plans and Eligibility](#plans-and-eligibility).

### 2. Bir başlangıç şablonu seçin

**Browse templates**'e tıklayın. Şunlardan birini seçin:

- [Moderator](#template-moderator) - işaretlenen veya yeni yorumları gözden geçirir, ilk defa yorum yapanları uyarır, sadece uyarı sonrası yasaklamaya yükseltir.
- [Welcome Greeter](#template-welcome-greeter) - ilk defa yorum yapanlara cevap verir.
- [Top Comment Pinner](#template-top-comment-pinner) - oy eşiğini aşan anlamlı yorumları sabitler.
- [Thread Summarizer](#template-thread-summarizer) - uzun gönderilerde tarafsız bir özet yayınlar.

Her şablon, **Durum: Kuru Çalışma** seçili olarak önceden doldurulmuş bir düzenleme formuna gider.

### 3. Gözden geçirin ve kaydedin

Düzenleme formunda en az şunları yapın:

- **Internal name.** Yönetici panellerinde kullanılan kısa bir tanımlayıcı.
- **Display name.** Ajan yorum gönderdiğinde herkese görünen isim.
- **Initial prompt.** Şablonun istemini kendi üslubunuza ve özel kurallarınıza uyacak şekilde düzenleyin.
- **Approvals.** Gerçekleşmeden önce insan incelemesi gerektirmesi gereken eylemleri işaretleyin. Herhangi bir moderasyon tarzı ajan için en az `ban_user`'ı onay gerektiren olarak işaretlemenizi öneririz. Bkz. [Approval Workflow](#approval-workflow).

**Save agent**'e tıklayın.

### 4. Kuru çalışmada izleyin

Ajan artık **Kuru Çalışma** modunda yayında. Tetikleyicilerini alacak, modeli çağıracak ve [Run History](#run-history) sayfasında her satırda **Kuru Çalışma** rozeti ile işlemleri kaydedecek — ancak gerçek eylemler gerçekleştirmeyecek. Birkaç çalıştırma detayını ziyaret edin (bkz. [Run Detail View](#run-detail-view)) ve şunlara bakın:

- Ajanın seçtiği eylemler.
- Her eylemdeki gerekçe ve güven seviyesi.
- Tam LLM dökümü.

Ajanın verdiği kararlara katılmıyorsanız, başlangıç istemini düzenleyin veya daha fazla onay işaretleyin.

### 5. Geçmiş yorumlara karşı bir test çalıştırın

Ajanlar listesi sayfasından, ajanın satırında **Test run**'a tıklayın. Formda tek bir **Days** sayısal girişi vardır (1 ila 90). Örnek büyüklüğü ve değerlendirilen yorumlar üzerindeki sert üst sınır bilgi amaçlı gösterilir - bunlar sunucu tarafında hesaplanır, kullanıcı tarafından ayarlanmaz. Replay, gerçek eylemler gerçekleştirmeden geçmiş yorumlara karşı çalışır ve ajanın gerçekte ne yapacağını **yapmış olacağı** ile gerçekte olanı (yorumun daha sonra onaylanıp onaylanmadığı, spam olarak işaretlenip işaretlenmediği, silinip silinmediği vb.) raporlar. Bkz. [Test Runs (Replays)](#test-runs-replays).

### 6. Etkinleştirmeye geçin

Kuru çalışmadan ve replay çıktılarından memnunsanız, ajanı düzenleyin ve **Durum**u **Enabled** olarak değiştirin. Buradan itibaren gerçek eylemler uygulanır. Run History sayfası artık kuru çalışma rozeti olmadan canlı çalıştırmaları gösterir ve onay için işaretlediğiniz herhangi bir eylem [approvals inbox](#approval-workflow) içinde görünür.

### Sonraki adımlar

- [Budgets](#budgets-overview) ve [Budget Alerts](#budget-alerts) ayarlayın.
- Ajan olaylarına harici sistemlerin tepki vermesini istiyorsanız [Webhooks](#webhooks-overview) yapılandırın.
- Ajan kararlarını yazılı politikanızla uyumlu tutmak için [Community Guidelines](#community-guidelines) ekleyin.

---