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
    <!-- Cliente da API -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>client</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Biblioteca Core (inclui SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>0.0.2</version>
    </dependency>
    
    <!-- Biblioteca PubSub (para eventos ao vivo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>0.0.2</version>
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
    implementation "com.fastcomments:client:0.0.2"
    
    // Biblioteca Core (inclui SSO)
    implementation "com.fastcomments:core:0.0.2"
    
    // Biblioteca PubSub (para eventos ao vivo)
    implementation "com.fastcomments:pubsub:0.0.2"
}
```

### Library Contents

Esta biblioteca contém três módulos. O cliente de API gerado, a biblioteca Java core que contém utilitários escritos à mão
para facilitar o trabalho com a API, e o módulo `pubsub` que é uma biblioteca para se inscrever em feeds de alterações.

- [Documentação da biblioteca do cliente da API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentação da Biblioteca Core, incluindo exemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentação da Biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs Públicas vs APIs Seguras

Para o cliente de API, existem duas classes, `DefaultApi` e `PublicApi`. A `DefaultApi` contém métodos que exigem sua chave de API, e a `PublicApi` contém chamadas de API
que podem ser feitas diretamente de um navegador/dispositivo móvel/etc sem autenticação.