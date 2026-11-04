Por padrão, o FastComments não permite iframes nos comentários. Quando você ativa a incorporação de mídia, os comentaristas podem colar o código de incorporação (o trecho `<iframe>`) de provedores confiáveis como YouTube, Vimeo, SoundCloud e Spotify, e ele será renderizado inline no comentário.

Por segurança, esta não é uma flag de configuração do widget no lado do cliente. É uma configuração do lado do servidor, validada quando cada comentário é salvo, portanto não pode ser ativada a partir da página. Apenas iframes apontando para uma lista integrada de provedores confiáveis são permitidos. Qualquer outro iframe é removido.

Isso é feito sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.allow-embeds'; selector = '.allow-embeds'; alt='Configuração de incorporação de mídia ativada na página de personalização do widget, permitindo que comentaristas coletem incorporações de iframe confiáveis'; title='Permitir Incorporação de Mídia' app-screenshot-end]

### Adicionando Seus Próprios Provedores

Se você quiser permitir incorporações de um provedor que não está na lista integrada de confiáveis, adicione seu hostname no campo "Domínios de Incorporação Adicionais" na mesma página. Esses hostnames são permitidos além dos provedores integrados. A correspondência é exata, então inclua o hostname completo (por exemplo, player.example.com). Tudo que você não listar permanece bloqueado.

Tanto a caixa de comentário simples quanto o editor WYSIWYG suportam colar uma incorporação. No editor WYSIWYG, a incorporação é inserida como um bloco removível.