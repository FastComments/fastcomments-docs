---
### PyPI

```bash
pip install fastcomments
```

### Conteúdo da Biblioteca

Esta biblioteca contém dois módulos: o cliente de API gerado e a biblioteca principal em Python, que contém utilitários escritos à mão para tornar o trabalho com a API mais fácil, incluindo suporte a SSO.

- [Documentação da Biblioteca do Cliente de API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentação da Biblioteca Principal, Incluindo Exemplos de SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### APIs Públicas vs APIs Protegidas

Para o cliente de API, existem duas classes, `DefaultApi` e `PublicApi`. A `DefaultApi` contém métodos que requerem sua API key, e a `PublicApi` contém chamadas de API que podem ser feitas diretamente de um navegador/dispositivo móvel/etc sem autenticação.
---