FastComments LMS'inize kaydedildikten sonra, eğitmenler onu diğer herhangi bir LTI harici araç ekledikleri şekilde kurslara eklerler.

#### D2L Brightspace

Bir dersin içerik alanında:

1. **Add Existing Activities** > **External Learning Tools** öğesine tıklayın.
2. Listeden **FastComments** öğesini seçin.
3. Araç içerik alanında bir konu olarak görünür. Bu kaynağa ait yorum dizisini başlatmak için onu bir kez açın.

#### Moodle

Bir kursta:

1. **Düzenleme modunu** açın.
2. Yorumların görünmesini istediğiniz bölümde **Etkinlik veya kaynak ekle** öğesine tıklayın.
3. Aktivite seçicisinden **FastComments**'ı seçin.
4. Kaydedin. Öğrenciler yorum dizisini bölüm içinde görecekler.

#### Blackboard Learn

Bir kursta:

1. Bir İçerik Alanına gidin.
2. **İçerik Oluştur** > **FastComments** öğesine tıklayın (\"Öğrenme Araçları\" altında).
3. Bir ad belirleyin ve gönderin.

#### Sakai

Site yöneticileri aracı şu yoldan ekler: **Site Bilgisi** > **Araçları Yönet** > **Harici Araçlar** bölümüne kadar kaydırın > **FastComments**'ı seçin > **Devam**.

#### How Threads Are Scoped

FastComments her **(LMS örneği, ders, kaynak bağlantısı)** için ayrı bir yorum dizisi oluşturur. Bu şunları ifade eder:

- Aynı LMS içindeki iki farklı ders ayrı diziler alır, aynı araç adını kullansalar bile.
- Aynı derste iki farklı yerde kullanılan aynı FastComments aracı iki ayrı dizi oluşturur.
- Aynı FastComments hesabı altında bulunan iki farklı Brightspace kurulumu ayrı diziler alır - LMS host adı dizi tanımlayıcısının bir parçasıdır.

#### SSO

Öğrenciler bir giriş ekranı görmez. LMS, kimlik bilgilerini (isim, e-posta, avatar, rol) LTI başlatması yoluyla FastComments'e gönderir ve FastComments onları otomatik olarak oturum açmış kabul eder. Yorumları LMS hesaplarına atanır.

LMS'deki **Instructor** veya **Administrator** rollerine sahip kullanıcılar, dizi için FastComments'teki moderatör/yönetici rollerine otomatik olarak eşlenir.