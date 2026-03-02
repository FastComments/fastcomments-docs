#### Descargar el plugin

Descargue el ZIP de la última versión desde el <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">repositorio de FastComments Moodle en GitHub</a>.

#### Extraer en su directorio Moodle

Extraiga el ZIP dentro de su instalación de Moodle de modo que el plugin quede en `<moodle-root>/local/fastcomments`. El directorio del plugin debe contener `version.php`, `lib.php` y otros archivos del plugin directamente (no anidados en una subcarpeta).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalar desde el administrador de Moodle

Inicie sesión como administrador del sitio y vaya a **Administración del sitio > Notificaciones**. Moodle detectará el nuevo plugin y le pedirá que ejecute la instalación.

#### Configurar el plugin

Después de la instalación, vaya a **Administración del sitio > Plugins > Plugins locales > FastComments** para introducir sus ajustes. Consulte la sección [Configuración](#moodle-configuration) para obtener detalles sobre cada opción.