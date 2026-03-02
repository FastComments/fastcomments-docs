#### Baixar o Plugin

Baixe o ZIP da última versão do <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">repositório FastComments Moodle no GitHub</a>.

#### Extraia para o diretório do seu Moodle

Extraia o ZIP na sua instalação Moodle para que o plugin fique em `<moodle-root>/local/fastcomments`. O diretório do plugin deve conter `version.php`, `lib.php` e outros arquivos do plugin diretamente (não aninhados em uma subpasta).

Por exemplo:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalar via administrador do Moodle

Faça login como administrador do site e navegue até **Site Administration > Notifications**. O Moodle detectará o novo plugin e pedirá que você execute a instalação.

#### Configurar o Plugin

Após a instalação, vá para **Site Administration > Plugins > Local plugins > FastComments** para inserir suas configurações. Consulte a seção [Configuração](#moodle-configuration) para detalhes sobre cada opção.