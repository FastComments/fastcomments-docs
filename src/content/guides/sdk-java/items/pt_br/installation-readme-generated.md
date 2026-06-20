### Maven

Adicione o repositório Repsy ao POM do seu projeto:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

Then add the dependencies you need:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>2.0.0</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>2.0.0</version>
    </dependency>
</dependencies>
```

### Gradle

Adicione o repositório Repsy ao seu arquivo build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:2.0.0"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:2.0.0"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:2.0.0"
}
```

### Library Contents

Esta biblioteca contém três módulos. O cliente de API gerado, a biblioteca Java core que contém utilitários escritos à mão para facilitar o trabalho com a API, e o módulo `pubsub`, que é uma biblioteca para assinar feeds de alterações.

- [Documentação da API Client Library](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentação da Core Library, incluindo exemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentação da PubSub Library](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### Public vs Secured APIs

Para o cliente de API, há três classes, `DefaultApi`, `PublicApi`, e `ModerationApi`. O `DefaultApi` contém métodos que requerem sua chave de API, e o `PublicApi` contém métodos que podem ser executados diretamente de um navegador/dispositivo móvel/etc sem autenticação.

O `ModerationApi` alimenta o painel do moderador. Ele contém métodos para moderação de comentários (listar, contar, pesquisar, registros e exportar), ações de moderação (remover/restaurar, sinalizar, definir status de revisão/spam/aprovação, votos, e reabrir/fechar thread), banimentos (banir de comentar, desfazer um banimento, resumos pré-banimento, status e preferências de banimento, e contagens de usuários banidos), e badges & trust (conceder/remover um badge, badges manuais, obter/definir fator de confiança, e perfil interno do usuário). Todo método do `ModerationApi` aceita um parâmetro `sso` para que a chamada possa ser realizada em nome de um moderador autenticado via SSO.