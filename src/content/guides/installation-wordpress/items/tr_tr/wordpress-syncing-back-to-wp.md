Varsayılan olarak, FastComments WordPress sitenize günlük olarak geri senkronizasyon yapar. Bu tamamen yedekleme amaçlıdır, böylece verilerin bir kopyasının sahibi olmaya devam edersiniz ve buna bağlı olabilecek eklentiler için de gereklidir.

Bu, kaydedilen her yorumla anında gerçekleşmez; çünkü bazı siteler yoğun okuma trafiğini kaldırabilirken, veritabanı dağıtımları her zaman yoğun yazma trafiğini kaldıramaz (bu nedenle bu iş FastComments'e devredilir).

WordPress'e geri senkronizasyon zamanlaması bir eklenti kurarak özelleştirilebilir. [WP Crontrol](https://wordpress.org/plugins/wp-crontrol/#description) eklentisini öneriyoruz.

Adımlar:

1. WP Crontrol eklentisini kurun
2. `Settings -> Cron Schedules` bölümüne gidin.
3. `Cron Events` sekmesine gidin.
4. `fastcomments_cron_hook` araması yapın.
5. Olayı düzenleyin. Hook'u saatlik, günde iki kez, günlük (varsayılan) veya haftada bir çalışacak şekilde yapılandırabilirsiniz.

WordPress'e geri senkronizasyon, FastComments eklenti panosuna gidip `Manually Sync` seçeneğini belirleyerek herhangi bir zamanda manuel olarak da gerçekleştirilebilir. WP kurulumunuza geri senkronizasyon yapma veya WP yorumlarınızı FastComments sunucularına yeniden yükleme seçeneğine sahip olacaksınız.