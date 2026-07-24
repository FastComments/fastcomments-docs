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
        <version>3.0.1</version>
    </dependency>
    
    <!-- Biblioteca Core (inclui SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>3.0.1</version>
    </dependency>
    
    <!-- Biblioteca PubSub (para eventos ao vivo) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>3.0.1</version>
    </dependency>
</dependencies>
```

### Gradle

Adicione o repositório Repsy ao seu arquivo **build.gradle**:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // Cliente da API
    implementation "com.fastcomments:client:3.0.1"
    
    // Biblioteca Core (inclui SSO)
    implementation "com.fastcomments:core:3.0.1"
    
    // Biblioteca PubSub (para eventos ao vivo)
    implementation "com.fastcomments:pubsub:3.0.1"
}
```

### Conteúdo da Biblioteca

Esta biblioteca contém três módulos. O cliente de API gerado, a biblioteca Java core que contém utilitários escritos à mão para facilitar o trabalho com a API, e o módulo `pubsub`, que é uma biblioteca para assinar feeds de alterações.

- [Documentação da Biblioteca Cliente da API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentação da Biblioteca Core, Incluindo Exemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentação da Biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs Públicas vs Seguras

Para o cliente de API, há três classes, `DefaultApi`, `PublicApi` e `ModerationApi`. O `DefaultApi` contém métodos que requerem sua chave de API, e o `PublicApi` contém métodos que podem ser chamados diretamente de um navegador/dispositivo móvel/etc sem autenticação.

O `ModerationApi` fornece um conjunto extenso de APIs de moderação ao vivo e rápidas. Cada método do `ModerationApi` aceita um parâmetro `sso` e pode autenticar via SSO ou um cookie de sessão do FastComments.com.