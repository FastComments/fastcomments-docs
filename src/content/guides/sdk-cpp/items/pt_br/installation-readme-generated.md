### Instalar Dependências

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Compilando a partir do Código-Fonte

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

Esta biblioteca contém o cliente de API gerado e as utilidades SSO para facilitar o trabalho com a API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### APIs Públicas vs Seguras

Para o cliente de API, há três classes, `DefaultApi`, `PublicApi` e `ModerationApi`. O `DefaultApi` contém métodos que exigem sua chave de API, e o `PublicApi` contém
métodos que podem ser chamados diretamente de um navegador/dispositivo móvel/etc sem autenticação. O `ModerationApi` fornece um conjunto extenso de APIs de moderação em tempo real e rápidas. Cada método do `ModerationApi` aceita um parâmetro `sso` e pode autenticar via SSO ou um cookie de sessão do FastComments.com.