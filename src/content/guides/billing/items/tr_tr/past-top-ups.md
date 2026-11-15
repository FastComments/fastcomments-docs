Krediler sayfasının altındaki `Past top-ups` tablosu, en son yaptığınız kredi yüklemelerini yeniden eskiye doğru listeler; her biri için `Date`, `Amount` ve `Status` bilgilerini gösterir. İlk yüklemeden önce tabloda `No top-ups yet.` yazar.

Her yükleme üç durumdan birine sahiptir:

- `Credited`: Ödeme başarılı oldu ve tutar bakiyenize eklendi.
- `Pending`: Ödeme başlatıldı ancak tamamlanmadı; genellikle banka onayı tamamlanmadığı için. Bekleyen yüklemeler, Krediler sayfası bir sonraki açılışında Stripe üzerinden kontrol edilir ve `Credited` ya da `Failed` durumuna geçer.
- `Failed`: Kart reddedildi veya banka ödemeyi onaylamadı. Hiçbir ücret alınmadı ve bakiyeniz değişmedi.

Tablo, bu sayfadan yapılan yüklemeleri kaydeder. Kredi ile ödenen faturalar, faturaların kendileriyle birlikte [Billing Info](https://fastcomments.com/auth/my-account/billing-info) sayfasındaki `View Invoices` bölümünde gösterilir.