### Maven

Añada el repositorio Repsy al POM de su proyecto:

```xml
<repositories>
    <repository>
        <id>repsy</id>
        <name>FastComments Maven Repository on Repsy</name>
        <url>https://repo.repsy.io/mvn/winrid/fastcomments</url>
    </repository>
</repositories>
```

A continuación, agregue las dependencias que necesite:

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

Agregue el repositorio Repsy a su archivo build.gradle:

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

### Contenido de la biblioteca

Esta biblioteca contiene tres módulos. El cliente de API generado, la biblioteca Java principal que contiene utilidades escritas a mano para facilitar el trabajo con la API, y el módulo `pubsub` que es una biblioteca para suscribirse a flujos de cambios.

- [Documentación de la biblioteca del cliente de API](https://github.com/FastComments/fastcomments-java/blob/main/client/README.md)
- [Documentación de la biblioteca principal, incluidos ejemplos de SSO](https://github.com/FastComments/fastcomments-java/blob/main/core/README.md)
- [Documentación de la biblioteca PubSub](https://github.com/FastComments/fastcomments-java/blob/main/pubsub/README.md)

### APIs públicas vs seguras

Para el cliente de API, hay dos clases, `DefaultApi` y `PublicApi`. `DefaultApi` contiene métodos que requieren su clave API, y `PublicApi` contiene llamadas a la API que se pueden realizar directamente desde un navegador, dispositivo móvil, etc., sin autenticación.