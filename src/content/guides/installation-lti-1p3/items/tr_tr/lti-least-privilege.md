The FastComments LTI 1.3 entegrasyonu en az ayrıcalık ilkesine göre çalışır: yalnızca kullanıcıyı tanımlamak, yorumları doğru ders ve kaynağa iliştirmek ve rol tabanlı izinleri uygulamak için gereken başlatma taleplerini (launch claims) kullanır.

Bu sayfanın geri kalanı entegrasyonun tükettiği her talebi, talep etmediği her LTI Advantage servisini ve toplamadığı her veri kategorisini listeler. Güvenlik ve tedarik inceleyicileri cevapları doğrudan aşağıdaki tablolardan alabilir.

## LMS'den Alınan Veri Öğeleri

Every LTI 1.3 launch carries a signed JWT from the LMS. FastComments extracts the following claims from that JWT and uses nothing else:

| Field | LTI claim | Purpose | Required | Stored |
|-------|-----------|---------|----------|--------|
| User identifier | `sub` | Kullanıcıyı başlatmalar arasında tutarlı şekilde tanımlar, böylece aynı kişi aynı FastComments SSO kullanıcısına karşılık gelir | Evet | Evet, kararlı dahili bir SSO kimliğinin parçası olarak |
| Display name | `name` | Kullanıcının yorumlarının yanında gösterilen atıf | Evet (yoksa "LMS Kullanıcısı" olarak varsayılan) | Evet |
| Email | `email` | Hesap eşleştirme, bildirimler, moderasyon, destek yazışmaları | İsteğe bağlı (entegrasyon bunun olmadan da çalışır) | Sağlandığında evet |
| Avatar URL | `picture` | Kullanıcının yorumlarında gösterilir | İsteğe bağlı | Yalnızca URL; FastComments resmi indirmez veya yeniden barındırmaz |
| Roles | `https://purl.imsglobal.org/spec/lti/claim/roles` | Kullanıcının yönetici, eğitmen (moderator) veya öğrenen olup olmadığını belirler | Evet | SSO oturumunda türetilmiş `isAdmin` / `isModerator` bayrakları |
| Course context | `https://purl.imsglobal.org/spec/lti/claim/context` (`id`, `title`) | Yorum dizisini doğru LMS dersiyle ilişkilendirir | Evet | Evet, çözümlenmiş sayfa tanımlayıcısının bir parçası olarak |
| Resource link | `https://purl.imsglobal.org/spec/lti/claim/resource_link` (`id`) | Yorumları kurstaki doğru etkinlik veya araç yerleşimiyle ilişkilendirir | Mevcutsa evet | Evet, çözümlenmiş sayfa tanımlayıcısının bir parçası olarak |
| Deployment ID | `https://purl.imsglobal.org/spec/lti/claim/deployment_id` | Başlatmayı doğru FastComments kiracı yapılandırmasına yönlendirir | Evet | Evet, FastComments LTI yapılandırma kaydında |

## Kayıt Sırasında Bildirilen Talepler ve Kapsamlar

During LTI 1.3 Dynamic Registration, FastComments registers itself with `scope: ""` (no additional OAuth scopes) and declares only these OpenID Connect claims:

`iss`, `sub`, `name`, `email`, `picture`

It registers two message types:

- `LtiResourceLinkRequest` - FastComments'e yapılan standart ders başlatması.
- `LtiDeepLinkingRequest` - Eğitmenlerin FastComments aracını bir kurs içine yerleştirmesine olanak tanır.

No additional access tokens are requested from the LMS.

## LTI Advantage Services Not Requested

| Service / scope | Requested? | Reason |
|------------------|------------|--------|
| Names and Role Provisioning Services (NRPS) | Hayır | Entegrasyonun bir ders listesine ihtiyacı yok; kullanıcı kimliği her başlatma ile gelir |
| Assignment and Grade Services (AGS) - lineitem, score, result scopes | Hayır | Entegrasyon not defteri ile entegre değildir |
| Deep Linking beyond the standard placement return | Ek veri yok | Derin bağlantı yalnızca eğitmenlerin aracı yerleştirmesi için kullanılır; hiçbir ders içeriği listelenmez |

## Toplanmayan Veriler

Beyond LTI itself, FastComments does not request or receive the following from the LMS or user:

| Category | Collected? |
|----------|------------|
| Student grades | Hayır |
| Assignment submissions | Hayır |
| Attendance records | Hayır |
| Full course rosters | Hayır |
| Government identifiers | Hayır |
| Date of birth | Hayır |
| Postal address or phone number | Hayır |
| Financial information | Hayır |
| LMS administrator credentials | Hayır |

## Erişim Sınırları

- FastComments yalnızca LMS'nin kayıtlı anahtarlarıyla imzalanmış yetkili bir LTI 1.3 başlatması içinde veri alır. Entegrasyon ek bilgi için LMS'ye geri çağrı yapmaz.
- Başlatma belirteçleri tek kullanımlık ve kısa ömürlüdür. Tekrar oynatılan veya süresi dolmuş belirteçler reddedilir.
- LMS yöneticileri aracın platform içinde nerelere yerleştirileceğini kontrol eder. Örneğin D2L Brightspace, dağıtım başına org-birim kapsamı ve dağıtıma özel güvenlik ayarlarını destekler; bu, yöneticilerin aracı genel olarak erişilebilir kılmak yerine belirli derslere veya org birimlerine kısıtlamasına olanak tanır. Moodle, Blackboard, Sakai ve Schoology, LTI 1.3 uygulamalarında eşdeğer dağıtım-başı kontroller sunar.

## Depolama ve Saklama

FastComments, LTI'den türetilen verileri aktif yorumlama hizmetinin süresi boyunca ve müşteri tarafından yapılandırılmış saklama ayarlarına uygun şekilde tutar. Yorum verileri, dinlenme halindeyken şifrelenmiş üretim depolamada saklanır. Hesap sonlandırıldığında veya yazılı silme talebi alındığında, FastComments ilgili sözleşmeye uygun olarak müşteri verilerini siler veya anonimleştirir.

For full storage and data-handling details, see the <a href="https://fastcomments.com/privacy-policy" target="_blank">FastComments Gizlilik Politikası</a>.

## İnceleme Sıklığı

Any new LTI feature that would require additional claims, scopes, or LTI Advantage services is reviewed before release to confirm the requested access is necessary and proportionate to the feature being shipped.

## Güvenlik Anketleri için Kısa Açıklama

> FastComments, LTI 1.3 entegrasyonunda en az ayrıcalık ve veri minimizasyonu ilkesini uygular. Entegrasyon yalnızca kullanıcıyı kimlik doğrulamak (`sub`, `name`, `email`, `picture`), rolünü belirlemek ve yorumların ait olduğu ders ve kaynağı tanımlamak için gereken LTI başlatma taleplerini kullanır. FastComments İsim ve Rol Sağlama Servislerini, Ödev ve Not Servislerini, not defteri verilerini, devam kayıtlarını, tam ders listelerini veya LMS yönetici erişimini talep etmez. LMS yöneticileri, aracın hangi org birimlerinde, derslerde ve dağıtımlarda kullanılabilir olduğunu kontrol etmeye devam eder.