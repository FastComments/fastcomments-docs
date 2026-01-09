Bazen FastComments kullanıcılarınıza e-posta göndermek zorunda kalır, özellikle Secure SSO kullanmıyorsanız.

Buna örnek olarak, ilk kez yorum yaparken hesaplarını veya etkinliklerini doğrulama verilebilir. FastComments ayrıca yorumlarına gelen yanıtlar için bildirimler de gönderecektir.

When FastComments emails your users, we will use a default From Name and Email of `FastComments Robot` and `noreply@fastcomments.com`.

Ayrıca bu e-postaların altbilgisinde kendi logomuzu kullanacağız.

If you have FastComments Flex or Pro, this all can be customized on a per-domain basis via the "My Domains page":

[app-screenshot-start url='/auth/my-account/configure-domains'; selector = '.content form'; title='Customizing From Name, Email, and Logo' app-screenshot-end]

E-postalarda gösterilecek logoyu özelleştirirken, yüklediğiniz boyutun e-postanın altbilgisinde göstermek istediğiniz boyutla aynı olduğundan emin olun.

### `From Domain`'u Özelleştirirken

If you customize the `From Domain`, Email providers and clients need to know that FastComments is authorized to send emails on your behalf. Otherwise,
defining the `From Domain` and not following the below steps likely will result in emails going to spam.

#### 1. SPF Kurulumu

FastComments'in alan adınız adına güvenli bir şekilde e-posta göndermesine izin vermek için bize izin veren bir SPF kaydı eklediğinizden emin olun.

Ensure there are SPF records to allow `mail.fastcomments.com` and `sib.fastcomments.com` to send mail as your domain.

Bunu nasıl yapacağınıza dair daha fazla bilgi burada: https://mailtrap.io/blog/multiple-spf-records/

#### 2. DKIM Kurulumu

SPF'e ek olarak, DKIM'i de ayarlamalısınız. DNS yapılandırmanız hazır olduğunda, alan yapılandırmaları sayfasında DKIM ayarlarını alan başına göstermek için "Show Advanced"i tıklayabilirsiniz.

You can also [invoke the API](/guide-api.html#domain-config-structure) to set DKIM configuration.

### Abonelikten Çıkma Bağlantıları

When using SSO, the unsubscribe features used in emails and notifications can be customized [via the DomainConfigs API](/guide-api.html#domain-config-structure).

---