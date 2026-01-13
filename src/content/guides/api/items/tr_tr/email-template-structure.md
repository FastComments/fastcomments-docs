---
Bir `EmailTemplate` nesnesi, bir kiracı için özel bir e-posta şablonunun yapılandırmasını temsil eder.

Sistem, kullanılacak e-posta şablonunu şu yolla seçer:

- Türünü belirten tanımlayıcı, buna `emailTemplateId` diyoruz. Bunlar sabittir.
- `domain`. İlgili nesnenin (ör. bir `Comment`) bağlı olduğu domain için önce bir şablon bulmaya çalışacağız; eşleşme bulunmazsa domain'in null veya `*` olduğu bir şablon arayacağız.

`EmailTemplate` nesnesinin yapısı aşağıdaki gibidir:

[inline-code-attrs-start title = 'E-posta Şablon Yapısı'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface EmailTemplate {
    id: string
    tenantId: string
    emailTemplateId: string
    displayName: string
    /** SADECE OKUNUR **/
    createdAt: string
    /** SADECE OKUNUR **/
    updatedAt: string
    /** SADECE OKUNUR **/
    updatedByUserId: string
    /** Şablonun ilişkilendirileceği domain. **/
    domain?: string | '*' | null
    /** EJS sözdiziminde e-posta şablonu içeriği. **/
    ejs: string
    /** Her desteklenen yerel için geçersiz kılınmış çeviri anahtarlarının değerlerini içeren bir harita. **/
    translationOverridesByLocale: Record<string, Record<string, string>>
    /** Şablonun render bağlamını temsil eden bir nesne. **/
    testData: object
}
[inline-code-end]

### Notlar

- Geçerli `emailTemplateId` değerlerini `/definitions` uç noktasından alabilirsiniz.
- `/definitions` uç noktası ayrıca varsayılan çevirileri ve test verilerini içerir.
- Geçersiz yapı veya test verileri durumunda şablonlar kaydedilemez.

---