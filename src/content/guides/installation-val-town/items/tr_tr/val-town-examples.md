Bu rehberin her bir parçasını kapsayan, remixleyebileceğiniz dört herkese açık val.

**[Blog with comments](https://www.val.town/x/fastcomments/blog-with-comments)** ([canlı](https://fastcomments-blog.val.run)), her gönderinin altında bir yorum dizisi ve ana sayfada toplu yorum sayıları bulunan bir Markdown blogudur. Remixlediğiniz anda çalışır ve tek bir ortam değişkeni onu kendi hesabınıza yönlendirir.

**[SSO demo](https://www.val.town/x/fastcomments/sso-demo)** ([canlı](https://fastcomments-sso.val.run)), ziyaretçiyi Val Town hesabıyla oturum açar ve bu kimliği widget'a aktarır; böylece ikinci bir giriş gerekmez.

**[Webhook receiver](https://www.val.town/x/fastcomments/webhook-receiver)** ([canlı](https://fastcomments-webhooks.val.run)), her teslimatta HMAC imzasını doğrular ve olayları SQLite'ta saklar. Bir test yükünü imzalayıp kendine teslim eden bir düğmesi vardır; böylece gerçek bir webhook yapılandırmadan önce doğrulamanın başarılı olduğunu izleyebilirsiniz.

**[Agent skills](https://www.val.town/x/fastcomments/skills)** ([canlı](https://fastcomments-skills.val.run)), widget, SSO, REST API, moderasyon ve Disqus'tan geçiş konularını kapsayan FastComments agent skill'lerinden oluşan bir kütüphanedir. Remixlediğinizde Val Town'un agent'ı Townie, skill'leri `skills/` klasöründen otomatik olarak alır; böylece agent'ınız, dokümantasyonu sohbete yapıştırmanıza gerek kalmadan yorumları nasıl kuracağını bilir.

Aynı skill'ler başka her yerde `npx skills add fastcomments/skills` ile kurulabilir.