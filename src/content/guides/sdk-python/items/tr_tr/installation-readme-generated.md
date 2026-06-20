### PyPI

```bash
pip install fastcomments
```

### Library Contents

Bu kütüphane iki modül içerir: oluşturulmuş API istemcisi ve API ile çalışmayı kolaylaştıran, elle yazılmış yardımcılar içeren çekirdek Python kütüphanesi; SSO desteği dahil.

- [API İstemci Kütüphanesi Belgeleri](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Çekirdek Kütüphane Belgeleri, SSO Örnekleri Dahil](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs Secured APIs

API istemcisi için üç sınıf vardır: `DefaultApi`, `PublicApi`, ve `ModerationApi`. `DefaultApi` API anahtarınızı gerektiren yöntemleri içerir, ve `PublicApi` kimlik doğrulama olmadan doğrudan bir tarayıcı/mobil cihaz/vb üzerinden yapılabilecek yöntemleri içerir. `ModerationApi` moderatör panosunu çalıştırır ve yorumları denetlemek için yöntemler içerir (listele, say, ara, günlükler, dışa aktar), moderasyon eylemleri (kaldır/geri yükle, işaretle, inceleme/spam/onay durumunu ayarla, oylar, konuyu yeniden aç/kapat), yasaklamalar (yorumdan yasakla, geri al, ön-yasak özetleri, yasak durumu/tercihleri, yasaklı kullanıcı sayıları) ve rozetler & güven (rozet ver/kaldır, manuel rozetler, güven faktörünü al/ayarla, kullanıcı iç profili). Her `ModerationApi` yöntemi, SSO ile kimlik doğrulanmış bir moderatör adına çağrılabilmesi için bir `sso` parametresi kabul eder.