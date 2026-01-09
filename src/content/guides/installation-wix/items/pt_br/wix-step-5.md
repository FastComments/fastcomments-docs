A seguir, vamos configurar tudo para que o fio de comentários mude com base na página atual, permitindo que os usuários discutam o conteúdo exibido no momento.

Sem os passos a seguir, você terá apenas um fio de comentários global para todo o seu site — o que não é muito útil.

#### Modo Dev

Para adicionar essa funcionalidade, teremos que entrar no que o Wix chama de `Dev Mode`.

Clique na opção `Dev Mode` no topo da tela.

<div class="screenshot white-bg">
    <div class="title">Ativar Modo Dev</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-dev-mode.png" alt="Ativar Modo Dev" />
</div>

#### Definir o ID do Elemento

Vamos adicionar um código personalizado para conseguir isso, mas primeiro precisamos dar ao novo elemento embed um ID para nos referirmos a ele.

Vamos chamá-lo de `fastcomments`.

Clique no novo elemento embed que adicionamos e, no modo dev, no canto inferior direito, você deverá ver um campo de ID com um valor como `html1`:

<div class="screenshot white-bg">
    <div class="title">O campo ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-0.png" alt="O campo ID" />
</div>

Altere isto para `fastcomments` e pressione Enter:

<div class="screenshot white-bg">
    <div class="title">Definir o ID</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-5-id-1.png" alt="Definir o ID" />
</div>

Agora podemos adicionar nosso código personalizado que indica à área de comentários qual página estamos visualizando.

Na parte inferior da tela você deverá ver um editor de código como este:

<div class="screenshot white-bg">
    <div class="title">Abrir o Editor</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-open-editor.png" alt="Abrir o Editor" />
</div>

Copie o código a seguir e cole ali:

[inline-code-attrs-start title = 'Trecho de Navegação de Comentários do Wix'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import wixLocation from 'wix-location';

$w.onReady(function () {
    function updateFastCommentsLocation() {
        try {
            const url = (wixLocation.baseUrl + '/' + wixLocation.path).replace(/,/g, '/');
            $w('#fastcomments').postMessage({'action': 'reload', 'url': url});
        } catch (err) {
            console.error('Wix -> FastComments Error', err);
        }
    }

    updateFastCommentsLocation();
    wixLocation.onChange( () => {
        updateFastCommentsLocation();
    });
});
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Adicionar o Código de Navegação</div>
    <img class="screenshot-image" src="/images/installation-guides/installation-guide-wix-step-7-paste-code.png" alt="Adicionar o Código de Navegação" />
</div>

---