### Maven

Agrega el repositorio Repsy al POM de tu proyecto:

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
        <version>1.3.2</version>
    </dependency>
    
    <!-- Core Library (includes SSO) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>core</artifactId>
        <version>1.3.2</version>
    </dependency>
    
    <!-- PubSub Library (for live events) -->
    <dependency>
        <groupId>com.fastcomments</groupId>
        <artifactId>pubsub</artifactId>
        <version>1.3.2</version>
    </dependency>
</dependencies>
```

### Gradle

Agrega el repositorio Repsy al archivo build.gradle:

```groovy
repositories {
    mavenCentral()
    maven {
        url "https://repo.repsy.io/mvn/winrid/fastcomments"
    }
}

dependencies {
    // API Client
    implementation "com.fastcomments:client:1.3.2"
    
    // Core Library (includes SSO)
    implementation "com.fastcomments:core:1.3.2"
    
    // PubSub Library (for live events)
    implementation "com.fastcomments:pubsub:1.3.2"
}
```

### Contenido de la biblioteca

Esta biblioteca contiene tres módulos. El cliente de API generado, la biblioteca core de Java que contiene utilidades escritas a mano
para facilitar el trabajo con la API, y el módulo `pubsub` que es una biblioteca para suscribirse a feeds de cambios.

- [Documentación de la biblioteca del cliente de API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentación de la biblioteca core, incluyendo ejemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentación de la biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs públicas vs seguras

Para el cliente de API, hay dos clases, `DefaultApi` y `PublicApi`. `DefaultApi` contiene métodos que requieren tu API key, y `PublicApi` contiene llamadas a la API
que pueden realizarse directamente desde un navegador/dispositivo móvil/etc. sin autenticación.