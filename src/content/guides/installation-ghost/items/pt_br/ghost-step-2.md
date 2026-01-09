Agora que baixamos o arquivo zip, extraia-o para uma pasta. Eu baixei o `casper.zip` padrão e extraí para `Downloads\casper` no Windows.

Next, you'll want to make sure you have the LTS or newer version of NodeJS installed. You can get that here: https://nodejs.org/en/download/

Once NodeJS is installed, you'll want to install a code editor.

We recommend (and use) Webstorm, which you can get here with a 30-day trial (no credit card needed): https://www.jetbrains.com/webstorm/

The next best free option would probably be Visual Studio Code: https://code.visualstudio.com/download

Once you have your editor setup, and the theme folder open in the editor, open the terminal in the IDE and run:

[inline-code-attrs-start title = 'Instalar o Tema'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

A saída bem-sucedida será parecida com isto (você pode ignorar avisos):

<div class="screenshot white-bg">
    <div class="title">Saída bem-sucedida do npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Saída bem-sucedida do npm install" />
</div>

Isto configurará as dependências do tema para os comandos posteriores que iremos executar. Além disso, a exportação depende das dependências do tema estarem instaladas; caso contrário, a reimportação não funcionará corretamente.