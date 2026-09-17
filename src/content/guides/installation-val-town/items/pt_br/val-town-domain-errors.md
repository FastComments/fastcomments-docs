---
Ao desativar o locatário `demo`, o widget pode recusar o carregamento com um erro de autorização. Isso ocorre porque o FastComments não sabe que deve permitir que sua conta seja usada naquele domínio.

[Vá aqui para adicionar seu site à sua conta.](https://fastcomments.com/auth/my-account/configure-domains)

Val Town vale a pena observar novamente aqui, pois um val pode ser acessível por mais de um nome de host:

- Todo val HTTP tem um endpoint padrão longo, `<org>--<id>.web.val.run`.
- Reivindicar um subdomínio personalizado adiciona `<name>.val.run`.
- Um [domínio personalizado](https://docs.val.town/vals/http/custom-domains/) adiciona um terceiro.
- Ramos obtêm seus próprios URLs.

Adicione os nomes de host a partir dos quais você realmente serve o widget. Se você reivindicar um subdomínio depois de configurar tudo, adicione‑o também, caso contrário o widget funcionará na URL antiga e falhará na nova.
---