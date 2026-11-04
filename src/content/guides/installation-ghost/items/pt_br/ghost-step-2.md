Agora que baixamos o arquivo zip, extraia-o para uma pasta. Eu baixei o `casper.zip` padrão e extraí em `Downloads\casper` no Windows.

Em seguida, você vai querer garantir que tem a versão LTS ou mais recente do NodeJS instalada. Você pode obtê-la aqui: https://nodejs.org/en/download/

Depois que o NodeJS estiver instalado, você vai querer instalar um editor de código.

Recomendamos (e usamos) o Webstorm, que você pode obter aqui com um teste de 30 dias (sem necessidade de cartão de crédito): https://www.jetbrains.com/webstorm/

A próxima melhor opção gratuita provavelmente seria o Visual Studio Code: https://code.visualstudio.com/download

Depois de ter seu editor configurado e a pasta do tema aberta no editor, abra o terminal no IDE e execute:

[inline-code-attrs-start title = 'Instalar o tema'; type = 'bash'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
npm install
[inline-code-end]

A saída bem-sucedida ficará assim (você pode ignorar os avisos):

<div class="screenshot white-bg">
    <div class="title">Saída bem-sucedida do npm install</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-2-1-install.png" alt="Saída bem-sucedida do npm install" />
</div>

Isso configurará as dependências do tema para os comandos que executaremos mais tarde. Além disso, a exportação depende de as dependências do tema estarem instaladas, caso contrário a reimportação não funcionará corretamente.