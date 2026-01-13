Varsayılan olarak, her kullanıcı aynı dakika içinde en fazla `5 comments` gönderebilir.

Bu, user id, anon user id ve ip address (hashed) ile izlenir.

Bu, widget özelleştirme sayfasında kod yazmadan özelleştirilebilir:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Unutmayın ki, comment creation API'yi kullanıyorsanız, istekte kullanıcının orijinal `ip` adresini backend'imize göndermek isteyebilirsiniz, böylece rate limiting uygulanır
kullanıcı bazında ve hesabınıza genel olarak değil.