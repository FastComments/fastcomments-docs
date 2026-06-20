### Instalar Dependências

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Compilando a partir do código-fonte

```bash
mkdir build
cd build
cmake ..
make
```

### Instalando

```bash
sudo make install
```

### Conteúdo da Biblioteca

Esta biblioteca contém o cliente de API gerado e os utilitários SSO para facilitar o trabalho com a API.

- [Documentação da Biblioteca do Cliente da API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### APIs Públicas vs Seguras

Para o cliente de API, existem três classes, `DefaultApi`, `PublicApi` e `ModerationApi`. O `DefaultApi` contém métodos que requerem sua chave de API, e o `PublicApi` contém
métodos que podem ser chamados diretamente de um navegador/dispositivo móvel/etc sem autenticação. O `ModerationApi` contém métodos que alimentam o painel do moderador - listagem,
contagem, pesquisa, exportação e extração de logs de comentários, ações de moderação (remover/restaurar, sinalizar, definir status de revisão/spam/aprovação, ajustar votos, reabrir/fechar tópicos),
banimentos (banir a partir de um comentário, desfazer banimentos, resumos pré-banimento, status e preferências de banimento, contagens de usuários banidos), e distintivos & confiança (conceder/remover distintivos, distintivos manuais, obter/definir fator de confiança, perfil interno do usuário). Cada método do `ModerationApi` aceita um parâmetro `sso` para que a chamada seja realizada em nome de um moderador autenticado via SSO.