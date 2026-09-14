---
Four public vals you can remix, each covering one piece of this guide.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([live](https://fastcomments-blog.val.run)) bir Markdown blogudur, her gönderinin altında bir konu başlığı ve indeksde toplu yorum sayıları bulunur. Karıştırdığınız anda çalışır ve bir ortam değişkeni onu kendi hesabınıza yönlendirir.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([live](https://fastcomments-sso.val.run)) ziyaretçiyi Val Town hesabıyla oturum açtırır ve bu kimliği widget'a verir, böylece ikinci bir oturum açma gerekmez.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([live](https://fastcomments-webhooks.val.run)) her teslimatta HMAC imzasını doğrular ve olayları SQLite'da saklar. Test bir yük imzalayan ve kendisine teslim eden bir düğmesi vardır, böylece gerçek bir webhook yapılandırmadan önce doğrulamanın başarılı olduğunu izleyebilirsiniz.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([live](https://fastcomments-skills.val.run)) FastComments ajan becerilerini içeren bir kütüphanedir; widget, SSO, REST API, moderasyon ve Disqus'tan geçişi kapsar. Bunu karıştırdığınızda Val Town'un ajanı Townie, `skills/` içindeki becerileri otomatik olarak alır, böylece ajanınız sohbet içine belge yapıştırmadan yorumları nasıl bağlayacağını bilir.

Aynı beceriler, `npx skills add fastcomments/skills` komutuyla başka bir yerde de kurulabilir.
---