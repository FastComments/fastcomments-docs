### PyPI

```bash
pip install fastcomments
```

### Conteúdo da Biblioteca

Esta biblioteca contém dois módulos: o cliente de API gerado e a biblioteca principal em Python, que contém utilitários escritos à mão para facilitar o trabalho com a API, incluindo suporte a SSO.

- [Documentação da biblioteca do cliente API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentação da biblioteca principal, incluindo exemplos de SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### APIs Públicas vs Seguras

Para o cliente de API, existem três classes, `DefaultApi`, `PublicApi`, e `ModerationApi`. A `DefaultApi` contém métodos que requerem sua chave de API, e a `PublicApi` contém métodos que podem ser feitos diretamente de um navegador/dispositivo móvel/etc sem autenticação. A `ModerationApi` alimenta o painel do moderador e contém métodos para moderar comentários (listar, contar, buscar, logs, exportar), ações de moderação (remover/restaurar, sinalizar, definir status de revisão/spam/aprovação, votos, reabrir/fechar tópico), banimentos (proibir de comentar, desfazer, resumos pré-banimento, status/preferências de banimento, contagem de usuários banidos), e crachás & confiança (conceder/remover crachá, crachás manuais, obter/definir fator de confiança, perfil interno do usuário). Todo método da `ModerationApi` aceita um parâmetro `sso` para que possa ser chamado em nome de um moderador autenticado via SSO.