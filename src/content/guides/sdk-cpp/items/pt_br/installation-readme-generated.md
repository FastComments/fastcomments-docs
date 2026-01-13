### Instalar Dependências

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Construindo a partir do código-fonte

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

- [Documentação da Biblioteca do Cliente de API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### APIs Públicas vs Protegidas

Para o cliente de API, há duas classes, `DefaultAPI` e `PublicAPI`. A `DefaultAPI` contém métodos que requerem sua chave de API, e `PublicAPI` contém chamadas de API que podem ser feitas diretamente de um navegador/dispositivo móvel/etc sem autenticação.