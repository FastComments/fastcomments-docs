#### Eklentiyi İndir

En son sürüm ZIP'ini <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub deposundan</a> indirin.

#### ZIP'i Moodle Dizininize Çıkarın

ZIP'i Moodle kurulumunuza çıkarın, böylece eklenti `<moodle-root>/local/fastcomments` konumunda olur. Eklenti dizini doğrudan `version.php`, `lib.php` ve diğer eklenti dosyalarını içermelidir (alt bir klasör içinde olmayacak şekilde).

Örneğin:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Moodle Yönetici Arayüzünden Yükleyin

Site yöneticisi olarak giriş yapın ve **Site Yönetimi > Bildirimler** sayfasına gidin. Moodle yeni eklentiyi algılar ve kurulum işlemini başlatmanız için sizi uyarır.

#### Eklentiyi Yapılandırın

Kurulumdan sonra ayarlarınızı girmek için **Site Yönetimi > Eklentiler > Yerel eklentiler > FastComments** bölümüne gidin. Her seçeneğin ayrıntıları için [Yapılandırma](#moodle-configuration) bölümüne bakın.