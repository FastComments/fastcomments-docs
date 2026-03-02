#### Eklentiyi İndirin

En son sürüm ZIP dosyasını <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">FastComments Moodle GitHub deposu</a>ndan indirin.

#### Moodle Dizininize Çıkartın

ZIP'i Moodle kurulumunuza çıkartın, böylece eklenti `<moodle-root>/local/fastcomments` içinde yer alır. Eklenti dizini doğrudan `version.php`, `lib.php` ve diğer eklenti dosyalarını içermelidir (alt klasöre gömülü olmamalıdır).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Moodle Yönetici Panelinden Yükleyin

Site yöneticisi olarak giriş yapın ve **Site Yönetimi > Bildirimler** yolunu izleyin. Moodle yeni eklentiyi algılayacak ve yüklemeyi çalıştırmanız için sizi yönlendirecektir.

#### Eklentiyi Yapılandırın

Yüklemeden sonra, ayarlarınızı girmek için **Site Yönetimi > Eklentiler > Yerel eklentiler > FastComments** bölümüne gidin. Her seçeneğin ayrıntıları için [Yapılandırma](#moodle-configuration) bölümüne bakın.