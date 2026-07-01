### Instalar a partir do GitHub

Instale diretamente a partir de uma tag de release (recomendado, totalmente reproduzível):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Fixe a tag em vez de um branch para que as builds sejam determinísticas. O mesmo formato funciona em `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Cada release marcado no [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) também tem um wheel compilado anexado, caso você prefira instalar um artefato binário diretamente.

### Conteúdo da Biblioteca

Esta biblioteca contém dois módulos: o cliente API gerado e a biblioteca principal Python que contém utilitários escritos à mão para facilitar o trabalho com a API, incluindo suporte a SSO.

- [Documentação da Biblioteca Cliente API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentação da Biblioteca Core, Incluindo Exemplos de SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### APIs Públicas vs Seguras

Para o cliente API, há três classes, `DefaultApi`, `PublicApi` e `ModerationApi`. O `DefaultApi` contém métodos que requerem sua chave de API, e o `PublicApi` contém métodos que podem ser chamados diretamente de um navegador/dispositivo móvel/etc sem autenticação. O `ModerationApi` fornece um conjunto extenso de APIs de moderação ao vivo e rápidas. Cada método do `ModerationApi` aceita um parâmetro `sso` e pode autenticar via SSO ou um cookie de sessão do FastComments.com.