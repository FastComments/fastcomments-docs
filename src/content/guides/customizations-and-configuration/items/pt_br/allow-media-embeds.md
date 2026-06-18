Por padrão o FastComments não permite iframes em comentários. Quando você habilita incorporações de mídia, os comentaristas podem colar o código de incorporação (o trecho `<iframe>`) de provedores confiáveis como YouTube, Vimeo, SoundCloud e Spotify, e ele será renderizado inline no comentário.

Por segurança, isto não é uma flag de configuração do widget no lado do cliente. É uma configuração do lado do servidor, validada quando cada comentário é salvo, portanto não pode ser ativada a partir da página. Somente iframes apontando para uma lista interna de provedores confiáveis são permitidos. Qualquer outro iframe é removido.

Isso é feito sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; title='Allow Media Embeds' app-screenshot-end]

### Adicionando Seus Próprios Provedores

Se você quiser permitir incorporações de um provedor que não esteja na lista interna de confiáveis, adicione seu hostname no campo "Domínios adicionais de incorporação" na mesma página. Esses hostnames são permitidos além dos provedores incorporados. A correspondência é exata, então inclua o hostname completo (por exemplo, player.example.com). Qualquer coisa que você não listar permanece bloqueada.

Tanto a caixa de comentário simples quanto o editor WYSIWYG suportam colar uma incorporação. No editor WYSIWYG a incorporação é inserida como um bloco removível.