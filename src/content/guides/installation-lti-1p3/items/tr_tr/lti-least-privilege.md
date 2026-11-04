FastComments LTI 1.3 entegrasyonu en az ayrıcalık ilkesine uyar: yalnızca kullanıcıyı tanımlamak, yorumları doğru ders ve kaynağa bağlamak ve rol tabanlı izinleri uygulamak için gereken lansman taleplerini (claims) kullanır.

Bu sayfanın geri kalanı, entegrasyonun tükettiği her talebi, talep etmediği her LTI Advantage servisini ve toplamadığı her veri kategorisini listeler. Güvenlik ve tedarik incelemecileri yanıtları doğrudan aşağıdaki tablolardan alabilir.

## LMS'den Alınan Veri Öğeleri

Her LTI 1.3 lansmanı, LMS'den imzalanmış bir JWT taşır. FastComments bu JWT'den aşağıdaki talepleri çıkarır ve başka hiçbir şeyi kullanmaz:

| Alan | LTI talebi | Amaç | Gerekli | Saklanır |
|-------|-----------|---------|----------|--------|
| Kullanıcı tanımlayıcı | `sub` | Kullanıcıyı lansmanlar arasında tutarlı şekilde tanımlar, böylece aynı kişi aynı FastComments SSO kullanıcısına eşlenir | Evet | Evet, kararlı bir dahili SSO kimliğinin parçası olarak |
| Görünen ad | `name` | Kullanıcının yorumlarının yanında gösterilen atıf | Evet (yoksa "LMS User" değerine düşer) | Evet |
| E-posta | `email` | Hesap eşleştirme, bildirimler, moderasyon, destek yazışmaları | İsteğe bağlı (entegrasyon olmadan da çalışır) | Sağlandığında evet |
| Avatar URL'si | `picture` | Kullanıcının yorumlarında görüntülenir | İsteğe bağlı | Yalnızca URL; FastComments görüntüyü indirmez veya yeniden barındırmaz |
| Roller | `https://purl.imsglobal.org/spec/lti/claim/roles` | Kullanıcının yönetici, eğitmen (moderatör) veya öğrenci olup olmadığını belirler | Evet | SSO oturumunda türetilmiş `isAdmin` / `isModerator` bayrakları |
| Ders bağlamı | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Yorum dizisini doğru LMS dersiyle ilişkilendirir | Evet | Evet, çözümlenen sayfa tanımlayıcısının parçası olarak |
| Kaynak bağlantısı | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Yorumları ders içindeki doğru etkinlik veya araç yerleşimiyle ilişkilendirir | Varsa evet | Evet, çözümlenen sayfa tanımlayıcısının parçası olarak |
| Dağıtım Kimliği | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Lansmanı doğru FastComments kiracı yapılandırmasına yönlendirir | Evet | Evet, FastComments LTI yapılandırma kaydında |

## Kayıt Sırasında Beyan Edilen Talepler ve Kapsamlar

LTI 1.3 Dinamik Kaydı sırasında FastComments, `scope: ""` (ek OAuth kapsamı yok) ile kendini kaydeder ve yalnızca şu OpenID Connect taleplerini beyan eder:

`iss`, `sub`, `name`, `email`, `picture`

İki mesaj türü kaydeder:

- `LtiResourceLinkRequest` - FastComments'e standart ders lansmanı.
- `LtiDeepLinkingRequest` - eğitmenlerin FastComments aracını bir dersin içine yerleştirmesine olanak tanır.

LMS'den ek erişim belirteci talep edilmez.

## Talep Edilmeyen LTI Advantage Servisleri

| Servis / kapsam | Talep edildi mi? | Neden |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | Hayır | Entegrasyonun ders listesine ihtiyacı yoktur; kullanıcı kimliği her lansmanla birlikte gelir |
| Assignment and Grade Services (AGS) - lineitem, score, result kapsamları | Hayır | Entegrasyon not defteri farkındalığına sahip değildir |
| Standart yerleşim dönüşünün ötesinde Deep Linking | Ek veri yok | Deep linking yalnızca eğitmenin aracı yerleştirmesi için kullanılır; ders içeriği listelenmez |

## Toplanmayan Veriler

LTI'nin ötesinde, FastComments LMS'den veya kullanıcıdan aşağıdakileri talep etmez veya almaz:

| Kategori | Toplanıyor mu? |
|----------|------------|
| Öğrenci notları | Hayır |
| Ödev teslimleri | Hayır |
| Devamsızlık kayıtları | Hayır |
| Tam ders listeleri | Hayır |
| Devlet tanımlayıcıları | Hayır |
| Doğum tarihi | Hayır |
| Posta adresi veya telefon numarası | Hayır |
| Finansal bilgiler | Hayır |
| LMS yönetici kimlik bilgileri | Hayır |

## Erişim Sınırları

- FastComments yalnızca LMS'nin kayıtlı anahtarlarıyla imzalanmış, yetkilendirilmiş bir LTI 1.3 lansmanı içindeki verileri alır. Entegrasyon ek bilgi için LMS'ye geri çağrı yapmaz.
- Lansman belirteçleri tek kullanımlık ve kısa ömürlüdür. Yeniden oynatılan veya süresi dolmuş belirteçler reddedilir.
- LMS yöneticileri aracın platformlarında nereye dağıtılacağını kontrol eder. Örneğin D2L Brightspace, dağıtım başına org-unit kapsamlandırmasını ve dağıtım başına güvenlik ayarlarını destekler; bu, yöneticilerin aracı genel kullanıma açmak yerine belirli derslere veya org unit'lere kısıtlamasına olanak tanır. Moodle, Blackboard, Sakai ve Schoology, LTI 1.3 uygulamalarında eşdeğer dağıtım başına kontroller sunar.

## Saklama ve Muhafaza

FastComments, LTI'den türetilen verileri aktif yorum servisinin süresi boyunca ve müşteri tarafından yapılandırılan saklama ayarlarına göre muhafaza eder. Yorum verileri, bekleme durumunda şifrelenmiş üretim depolamasında saklanır. Hesabın sonlandırılması veya yazılı silme talebi üzerine FastComments, ilgili sözleşmeye uygun olarak müşteri verilerini siler veya anonimleştirir.

Depolama ve veri işlemeyle ilgili tüm ayrıntılar için <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments Gizlilik Politikası</a> sayfasına bakın.

## İnceleme Sıklığı

Ek talepler, kapsamlar veya LTI Advantage servisleri gerektirecek her yeni LTI özelliği, yayınlanmadan önce istenen erişimin gerekli ve gönderilen özellikle orantılı olduğunu doğrulamak üzere incelenir.

## Güvenlik Anketleri İçin Kısa Bildirim

> FastComments, LTI 1.3 entegrasyonunda en az ayrıcalık ve veri minimizasyonu uygular. Entegrasyon, yalnızca kullanıcının kimliğini doğrulamak (`sub`, `name`, `email`, `picture`), rolünü belirlemek ve yorumların ait olduğu dersi ve kaynağı tanımlamak için gereken LTI lansman taleplerini kullanır. FastComments, Names and Role Provisioning Services, Assignment and Grade Services, not defteri verileri, devamsızlık, tam ders listeleri veya LMS yönetici erişimi talep etmez. LMS yöneticileri, aracın hangi org unit'lerde, derslerde ve dağıtımlarda kullanılabilir olduğunu kontrol etmeye devam eder.