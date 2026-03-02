#### Baixe o Plugin

Baixe o ZIP da última versão a partir do <a href="https://github.com/FastComments/fastcomments-moodle/" target="_blank">repositório FastComments Moodle no GitHub</a>.

#### Extraia para o Seu Diretório Moodle

Extraia o ZIP na sua instalação do Moodle de modo que o plugin fique em `<moodle-root>/local/fastcomments`. O diretório do plugin deve conter `version.php`, `lib.php`, e outros arquivos do plugin diretamente (não aninhados em uma subpasta).

For example:

    /var/www/html/moodle/local/fastcomments/version.php
    /var/www/html/moodle/local/fastcomments/lib.php
    /var/www/html/moodle/local/fastcomments/settings.php

#### Instalar pelo Administrador do Moodle

Faça login como administrador do site e navegue até **Administração do site > Notificações**. O Moodle detectará o novo plugin e solicitará que você execute a instalação.

#### Configure o Plugin

Após a instalação, vá para **Administração do site > Plugins > Plugins locais > FastComments** para inserir suas configurações. Consulte a seção [Configuração](#items-moodle-configuration) para detalhes sobre cada opção.