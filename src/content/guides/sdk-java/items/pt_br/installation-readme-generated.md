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

Em seguida, adicione as dependências que você precisa:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Biblioteca Core (inclui SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.1.2</version>
    </dependency>
    
    <!-- Biblioteca PubSub (para eventos ao vivo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.1.2</version>
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
    implementation "com.fastcomments:client:3.1.2"
    
    // Biblioteca Core (inclui SSO)
    implementation "com.fastcomments:core:3.1.2"
    
    // Biblioteca PubSub (para eventos ao vivo)
    implementation "com.fastcomments:pubsub:3.1.2"
}
```

### Conteúdo da Biblioteca

Esta biblioteca contém três módulos: o cliente de API gerado, a biblioteca Java core que contém utilitários escritos à mão
para facilitar o trabalho com a API, e o módulo `pubsub`, que é uma biblioteca para assinar feeds de alterações.

- [Documentação da Biblioteca API Client](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentação da Biblioteca Core, Incluindo Exemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentação da Biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs Públicas vs Protegidas

Para o cliente de API, existem três classes: `DefaultApi`, `PublicApi` e `ModerationApi`. A `DefaultApi` contém métodos que exigem sua chave de API, e a `PublicApi` contém métodos
que podem ser chamados diretamente de um navegador/dispositivo móvel/etc sem autenticação.

A `ModerationApi` fornece um conjunto extenso de APIs de moderação rápidas e ao vivo. Cada método da `ModerationApi` aceita um parâmetro `sso` e pode autenticar via SSO ou por um cookie de sessão do FastComments.com.