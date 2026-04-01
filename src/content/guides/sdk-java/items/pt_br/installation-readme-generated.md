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
    <!-- Cliente da API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Biblioteca Core (inclui SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- Biblioteca PubSub (para eventos ao vivo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
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
    // Cliente da API
    implementation "com.fastcomments:client:1.3.2"
    
    // Biblioteca Core (inclui SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // Biblioteca PubSub (para eventos ao vivo)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Conteúdo da Biblioteca

Esta biblioteca contém três módulos. O cliente de API gerado, a biblioteca Java core que contém utilitários escritos manualmente para facilitar o trabalho com a API, e o módulo `pubsub`, que é uma biblioteca para assinar feeds de alteração.

- [Documentação da Biblioteca do Cliente da API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentação da Biblioteca Core, incluindo exemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentação da Biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs Públicas vs APIs Protegidas

Para o cliente de API, existem duas classes, `DefaultApi` e `PublicApi`. A `DefaultApi` contém métodos que requerem sua chave de API, e a `PublicApi` contém chamadas de API que podem ser feitas diretamente de um navegador/dispositivo móvel/etc sem autenticação.