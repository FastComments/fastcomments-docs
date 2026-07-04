---
Oturum açmış kullanıcının widget'a otomatik olarak aktarılması için, etiketler mevcut kullanıcıyı isteği üzerinden okur. Projenizin aşağıdakilere sahip olduğundan emin olun (standart bir Django projesinde varsayılan olarak etkindir):

- `django.template.context_processors.request` in `TEMPLATES["OPTIONS"]["context_processors"]`
- `django.contrib.auth.middleware.AuthenticationMiddleware` in `MIDDLEWARE`

Şablon bağlamında bir request olmadığında, widget'lar anonim bir ziyaretçi için render edilir. Her zaman bir kullanıcıyı açıkça geçirebilirsiniz: `{% fastcomments user=some_user %}`.
---