**Moodle mı kullanıyorsunuz?** Ayrıca FastComments için LTI 1.3'ten daha sıkı bir entegrasyon sunan (not eşitleme kancaları, daha kapsamlı etkinlik raporlaması, yerel Moodle ayarlar kullanıcı arayüzü) özel bir Moodle eklentisi yayımlıyoruz. <a href="/guide-installation-moodle.html" target="_blank">Moodle eklenti kurulum kılavuzuna</a> bakın. Aşağıdaki LTI 1.3 akışı, diğer LMS'leri de kapsayan tek bir kayıt istiyorsanız veya Moodle yöneticiniz üçüncü taraf eklentileri yüklemeyecekse doğru seçimdir.

Moodle 4.0+ External Tool eklentisi aracılığıyla LTI 1.3 Dinamik Kayıt'ı destekler.

#### Araç Yönetim Ekranını Açın

1. Moodle'a site yöneticisi olarak giriş yapın.
2. **Site yönetimi** > **Eklentiler** > **Etkinlik modülleri** > **External tool** > **Araçları yönet** yolunu izleyin.

#### URL'yi Yapıştırın

**Tool URL** başlıklı bir kart göreceksiniz. Metin alanına FastComments kayıt URL'sini (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">buradan alın</a>) yapıştırın ve **Add LTI Advantage**'a tıklayın.

Moodle, aracın kimliğini ve talep ettiği izinleri gösteren bir kayıt ekranı açar. İnceleyin ve Moodle sürümüne bağlı olarak **Etkinleştir** (veya **Kaydet**) seçeneğine tıklayın.

Kayıt tamamlandığında açılır pencere kapanır; yeni FastComments aracı **Araçlar** listesinde **Etkin** durumuyla görünür.

#### Kullanılabilir Hale Getirin

Varsayılan olarak Moodle yeni araçları "Course tools" listesine ekler ancak etkinlik seçicisinde göstermez. FastComments'ı tüm ders genelinde görünür yapmak için:

1. FastComments kartındaki dişli simgesine tıklayın.
2. **Tool configuration usage** altında **Show in activity chooser and as a preconfigured tool** seçeneğini işaretleyin.
3. Kaydet.

Eğitmenler artık herhangi bir derse **Etkinlik veya kaynak ekle** > **FastComments** yoluyla FastComments ekleyebilir.