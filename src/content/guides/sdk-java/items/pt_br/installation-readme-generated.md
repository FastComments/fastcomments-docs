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

Em seguida, adicione as dependências necessárias:

```xml
<dependencies>
    <!-- API Client -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.1</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.1</version>
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
    implementation "com.fastcomments:client:1.3.1"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.1"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.1"
}
```

### Conteúdo da Biblioteca

Esta biblioteca contém três módulos. O cliente de API gerado, a biblioteca Java core que contém utilitários escritos à mão para facilitar o trabalho com a API, e o módulo `pubsub`, que é uma biblioteca para assinar feeds de alterações.

- [Documentação da Biblioteca do Cliente de API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentação da Biblioteca Core, incluindo Exemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentação da Biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs Públicas vs Seguras

Para o cliente de API, existem duas classes, `DefaultApi` e `PublicApi`. A `DefaultApi` contém métodos que exigem sua chave de API, e a `PublicApi` contém chamadas de API que podem ser feitas diretamente de um navegador/dispositivo móvel/etc. sem autenticação.