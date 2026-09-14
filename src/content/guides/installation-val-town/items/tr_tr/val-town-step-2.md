`tenantId: "demo"` ortak bir halka açık sandbox'tur. Kayıt olmadan çalışır, bu yüzden örneklerde kullanılır, ancak FastComments deneyen diğer herkes aynı konulara yazar ve herkes bunları denetleyebilir. Önemli bir şeyi yayınlamadan önce değiştirin.

Kiracı kimliğiniz [API gizli sayfasında](https://fastcomments.com/auth/my-account/api-secret) bulunur.

Bir kiracı kimliği herkese açıktır ve tarayıcı kodunda bulunur. API gizli anahtarı böyle değildir ve bu sayfadaki hiçbir şey ona ihtiyaç duymaz.

## Bir ortam değişkeninden okuyun

Val Town değerleri ücretsiz katmanda herkese açıktır, bu yüzden kaynakları herkes tarafından okunabilir. Hassas tüm verileri ortam değişkenlerinde tutun, `Deno.env.get` ile okuyun:

[inline-code-attrs-start title = 'config.ts'; type='javascript' inline-code-attrs-end]
[inline-code-start]
export const TENANT_ID = Deno.env.get("FASTCOMMENTS_TENANT_ID") ?? "demo";

// Only accounts created on eu.fastcomments.com set this, to "eu".
export const REGION = Deno.env.get("FASTCOMMENTS_REGION") ?? "";

export const CDN = REGION === "eu"
  ? "https://cdn-eu.fastcomments.com"
  : "https://cdn.fastcomments.com";
[inline-code-end]

Bu, Val Town'da ikinci bir neden dolayı genellikle olduğundan daha önemlidir: **bir değeri yeniden karıştırmak ortam değişkeni anahtarlarını kopyalar, ancak değerlerini kopyalamaz.** Ortam değişkeninde tutulan bir gizli anahtar, değerinizi başka birinin hesabına taşımaz. Bir dosyaya yazılan bir gizli anahtar ise taşır.

`"demo"`'ya geri dönmek, değeri kendi kiracısını ayarlamadan önce yeniden karıştıran herkes için çalışır durumda tutar.

## AB hesapları

Bir hesap, verileri ve anahtarları tek bir bölgede bulunur. Eğer hesabınız `eu.fastcomments.com` üzerinde oluşturulduysa, her widget yapılandırması da `region: "eu"` gerektirir ve betikler `cdn-eu.fastcomments.com` adresinden yüklenir. Aksi takdirde ikisini de olduğu gibi bırakın.

---